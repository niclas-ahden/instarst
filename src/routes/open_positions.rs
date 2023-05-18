use leptos::*;
use leptos::html::*;
use leptos_router::*;

#[component]
pub fn OpenPositions(cx: Scope) -> impl IntoView {
    view! { cx,
        <h2>"Open Positions"</h2>
        <ul>
            <li>
                <A href="/open-positions/typescript-front-end-developer">
                    "TypeScript Front-end utvecklare"
                </A>
            </li>
            <li>
                <A href="/open-positions/typescript-back-end-developer">
                    "TypeScript Back-end utvecklare"
                </A>
            </li>
        </ul>
    }
}

#[component]
pub fn FrontEndDeveloper(cx: Scope) -> impl IntoView {
    view! { cx,
        <h2>"TypeScript Front-end utvecklare"</h2>

        <h3>"Om tjänsten"</h3>
        <p>"Du kommer att utveckla Sveriges bästa skyltprogram och få möjlighet att göra det från grunden. Vi kommer att ersätta vår existerande produkt med en ny webbaserad version och den får du vara med och ta fram."</p>
        <p>"Du blir briefad på vad vi skall bygga och så sätter vi igång med teknikval såsom ramverk, lint-regler etc. och gör allt såsom vi vill ha det. Du bygger ett modernt, användarvänligt och snabbt gränssnitt för att designa skyltmallar (tänk www.canva.com). Allt är nytt men du kan luta dig mot vår existerande produkt för inspiration på vad som behöver byggas. Samtidigt har vi en gedigen kundstock för att samla in feedback från faktiska användare."</p>
        <p>"Servern skrivs också i TypeScript (ramverk kommer att väljas gemensamt), så kod kan delas och kommunikation blir enklare. Back-end-utvecklaren rekryteras nu parallellt så ni kommer att börja ungefär samtidigt. Ni omges av seniora kollegor med djup domänkunskap, så ni bygger nytt med tillgång till utvecklare, designers och supportpersonal som har flera års erfarenhet av våra kunder och deras behov."</p>
        <p>"Rent praktiskt så kommer butikspersonal hos Sveriges kändaste retail-kedjor med flera att använda skyltprogrammet du bygger för att skapa skyltar i sina butiker. När du går och handlar kommer du att se något som printats eller lagts upp på en digital skylt från din applikation."</p>
        <p>"Vi tror att vi har möjlighet att bygga en modern, välstrukturerad applikation som utan överdrift kommer att vara bäst på marknaden. Det är sällsynt att få göra detta från scratch, men nu händer det och vi vill att du hjälper oss!"</p>

        <h3>"Är du rätt för oss?"</h3>
        <ul>
            <li>"Flerårig erfarenhet av utveckling (med eller utan utbildning)"</li>
            <li>"Bekväm att fatta tekniska beslut och långsiktigt äga utfallen"</li>
            <li>"Gedigen erfarenhet av att utveckla komplicerade men användarvänliga gränssnitt"</li>
            <li>"God kommunikationsförmåga och vilja att samarbeta"</li>
            <li>"Driven att göra applikationen bättre; det gnager om det finns långsiktiga buggar eller tillkortakommanden"</li>
            <li>"Att en användare hör av sig om att du löst deras praktiska problem är roligare än att hoppa på nästa JS-ramverk"</li>
        </ul>

        <h3>"Meriterande kunskaper"</h3>
        <ul>
            <li>"UX/UI-design"</li>
            <li>"Intresse av grafisk design (om du arbetat mycket med ett grafiskt program så förstår du stora delar av vad vår applikation måste stödja för att designa en skylt)"</li>
            <li>"Full-stack eller intresse utöver front-end (vill du arbeta full-stack eller utvecklas inom back-end så finns den möjligheten här)"</li>
        </ul>

        <ApplicationForm role=Role::FrontEnd />
    }
}

#[component]
pub fn BackEndDeveloper(cx: Scope) -> impl IntoView {
    view! { cx,
        <h2>"TypeScript Back-end utvecklare"</h2>
        <ApplicationForm role=Role::BackEnd />
    }
}


#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub enum Role {
    FrontEnd,
    BackEnd
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Application {
    role: Role,
    name: String,
    email: String,
    text: String,
}

#[component]
fn ApplicationForm(cx: Scope, role: Role) -> impl IntoView {
    let name_input: NodeRef<Input> = create_node_ref(cx);
    let email_input: NodeRef<Input> = create_node_ref(cx);
    let text_input: NodeRef<Textarea> = create_node_ref(cx);

    let on_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        let name = name_input().expect("name input to exist").value();
        let email = email_input().expect("email input to exist").value();
        let text = text_input().expect("text input to exist").value();
        let application = Application { role: role.clone(), name, email, text };
        spawn_local(async {
            let _ = apply_for_role(application).await;
        });
    };

    view! { cx,
        <form class="ApplicationForm" on:submit=on_submit>
            <Input type_="text" label="Namn" required=true node_ref=name_input />
            <Input type_="email" label="E-post" required=true node_ref=email_input />
            <Textarea label="Meddelande" node_ref=text_input />
            <input class="ApplicationForm-submit" type="submit" value="Skicka ansökan" />
        </form>
    }
}

#[component]
fn Input(
    cx: Scope,
    #[prop(into)]
    label: String,
    #[prop(into)]
    type_: String,
    required: bool,
    node_ref: NodeRef<Input>
) -> impl IntoView {
    view! { cx,
        <label class="ApplicationForm-label">
            {label}
            <input
                class="ApplicationForm-input"
                prop:type={type_}
                prop:required={required.then_some("required")} // TODO: This can't be idiomatic.
                node_ref=node_ref
            />
        </label>
    }
}

#[component]
fn Textarea(
    cx: Scope,
    #[prop(into)]
    label: String,
    node_ref: NodeRef<Textarea>
) -> impl IntoView {
    view! { cx,
        <label class="ApplicationForm-label">
            {label}
            <textarea class="ApplicationForm-textarea" node_ref=node_ref />
        </label>
    }
}

#[server(ApplyForRole, "/api")]
pub async fn apply_for_role(application: Application) -> Result<(), ServerFnError> {
    // TODO: Persist the application.
    println!("Application received: {:?}", application);
    Ok(())
}
