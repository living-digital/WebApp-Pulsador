use yew::prelude::*;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use anyhow::Error;
use web_sys::window;

enum Msg {
    OpenDoor,
    ResponseReceived(Result<String, Error>),
}

struct App {
    feedback: Option<String>,
    token: Option<String>,
}

//Url /?token=hacker


impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        // Extraemos el token de la query string de la URL, ej: ?token=CLIENT_TOKEN_UNICO
        let token = window()
            .and_then(|w| w.location().search().ok())
            .and_then(|search| {
                // "search" tendría formato "?token=CLIENT_TOKEN_UNICO"
                let params = web_sys::UrlSearchParams::new_with_str(&search).ok()?;
                params.get("token")
            });

        Self {
            feedback: None,
            token,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::OpenDoor => {
                // Tomamos el token extraído; si no lo hubiera, usamos "no-token"
                let token = self.token.clone().unwrap_or_else(|| "no-token".to_string());
                // Formamos la URL al endpoint de Node-RED, agregando el token como query parameter
                let url = format!(
                    "http://devel.livingdigitalsolutions.com/admin/pulsador?token={}",
                    token
                );

                let link = ctx.link().clone();
                spawn_local(async move {
                    // Enviamos la petición POST
                    let result = Request::post(&url).send().await;
                    match result {
                        Ok(resp) => {
                            // Obtenemos el texto de la respuesta (si llega correctamente)
                            let text = resp
                                .text()
                                .await
                                .unwrap_or_else(|_| "Sin respuesta".to_string());
                            link.send_message(Msg::ResponseReceived(Ok(text)));
                        }
                        Err(err) => {
                            // Si ocurre un error de red (NetworkError, CORS, etc.), lo guardamos en el estado
                            link.send_message(Msg::ResponseReceived(Err(err.into())));
                        }
                    }
                });
                false
            }
            Msg::ResponseReceived(result) => {
                // Actualizamos `feedback` con el mensaje (éxito o error) para mostrarlo en la web
                self.feedback = Some(match result {
                    Ok(msg) => msg,
                    Err(err) => format!("Error: {}", err),
                });
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <section class="hero is-fullheight">
                <div class="hero-head">
                    <nav class="navbar">
                        <div class="navbar-brand">
                            <a class="navbar-item" href="#">
                                <strong>{ "Control de Acceso" }</strong>
                            </a>
                        </div>
                    </nav>
                </div>
                <div class="hero-body">
                    <div class="container has-text-centered">
                        <h1 class="title is-1">{ "Abrir la Puerta" }</h1>
                        <p class="subtitle is-3">{ "Presiona el botón para enviar la señal de apertura" }</p>
                        <button class="door-button" onclick={ctx.link().callback(|_| Msg::OpenDoor)}>
                            { "Abrir" }
                        </button>
                        {
                            // Aquí se muestra la respuesta o error devuelto por Node-RED (o el error de red)
                            if let Some(ref feedback) = self.feedback {
                                html! { <div class="notification is-primary mt-5">{ feedback }</div> }
                            } else {
                                html! {}
                            }
                        }
                    </div>
                </div>
            </section>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
