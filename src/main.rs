use yew::prelude::*;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use anyhow::Error;

enum Msg {
    OpenDoor,
    ResponseReceived(Result<String, Error>),
}

struct App {
    feedback: Option<String>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { feedback: None }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::OpenDoor => {
                // Clonamos el link para enviar mensajes desde la tarea asíncrona
                let link = ctx.link().clone();
                // Llamada asíncrona al endpoint configurado para abrir la puerta
                spawn_local(async move {
                    // Cambia la URL al endpoint que tengas configurado (por ejemplo, en Node-RED)
                    let result = Request::post("http://devel.livingdigitalsolutions.com/admin/pulsador")
                        .send()
                        .await;
                    
                    match result {
                        Ok(resp) => {
                            let text = resp.text().await.unwrap_or_else(|_| "Sin respuesta".to_string());
                            link.send_message(Msg::ResponseReceived(Ok(text)));
                        }
                        Err(err) => link.send_message(Msg::ResponseReceived(Err(err.into()))),
                    }
                });
                false
            }
            Msg::ResponseReceived(result) => {
                self.feedback = Some(match result {
                    Ok(msg) => format!("Respuesta del servidor: {}", msg),
                    Err(err) => format!("Error: {}", err),
                });
                true  // Se re-renderiza para actualizar la UI
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
                 /*  {
                    if let Some(ref feedback) = self.feedback {
                      html! { <div class="notification is-primary mt-5">{ feedback }</div> }
                    } else {
                      html! {}
                    }
                  }*/
                </div>
              </div>
            </section>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
