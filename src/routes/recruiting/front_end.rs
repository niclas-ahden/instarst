use leptos::*;

#[component]
pub fn FrontEndDeveloper(cx: Scope) -> impl IntoView {
    view! { cx,
        <section class="JobDescription">
            <h2>"Front-end utvecklare — TypeScript"</h2>
            <h3>"Om företaget"</h3>
            <p>
                "Väletablerat och marknadsledande företag i Tyresö som hjälper landets största retailkedjor med skyltning i butik. De erbjuder en användarvänlig och egenutvecklad skyltprogramvara som används av huvudkontor och butikspersonal för att skapa skyltar, skriva ut allt från etiketter till stora banderoller och att styra digital skyltning i tusentals butiker."
            </p>
            <p>
                "De är 34 anställda, varav 10 på IT-avdelningen. Alla känner varandra och har stor inverkan."
            </p>
            <h3>"Om tjänsten"</h3>
            <p>
                "Du kommer att utveckla Sveriges bästa skyltprogram och få möjlighet att göra det från grunden. Vi kommer att ersätta vår existerande produkt med en ny webbaserad version och den får du vara med och ta fram."
            </p>
            <p>
                "Du blir briefad på vad vi skall bygga och så sätter vi igång med teknikval såsom ramverk, lint-regler etc. och gör allt såsom vi vill ha det. Du bygger ett modernt, användarvänligt och snabbt gränssnitt för att designa skyltmallar (tänk www.canva.com). Allt är nytt men du kan luta dig mot vår existerande produkt för inspiration på vad som behöver byggas. Samtidigt har vi en gedigen kundstock för att samla in feedback från faktiska användare."
            </p>
            <p>
                "Servern skrivs också i TypeScript (ramverk kommer att väljas gemensamt), så kod kan delas och kommunikation blir enklare. Back-end-utvecklaren rekryteras nu parallellt så ni kommer att börja ungefär samtidigt. Ni omges av seniora kollegor med djup domänkunskap, så ni bygger nytt med tillgång till utvecklare, designers och supportpersonal som har flera års erfarenhet av våra kunder och deras behov."
            </p>
            <p>
                "Rent praktiskt så kommer butikspersonal hos Sveriges kändaste retail-kedjor med flera att använda skyltprogrammet du bygger för att skapa skyltar i sina butiker. När du går och handlar kommer du att se något som printats eller lagts upp på en digital skylt från din applikation."
            </p>
            <p>
                "Vi tror att vi har möjlighet att bygga en modern, välstrukturerad applikation som utan överdrift kommer att vara bäst på marknaden. Det är sällsynt att få göra detta från scratch, men nu händer det och vi vill att du hjälper oss!"
            </p>
            <h3>"Är du rätt för oss?"</h3>
            <ul>
                <li>"Flerårig erfarenhet av utveckling (med eller utan utbildning)"</li>
                <li>"Bekväm att fatta tekniska beslut och långsiktigt äga utfallen"</li>
                <li>
                    "Gedigen erfarenhet av att utveckla komplicerade men användarvänliga gränssnitt"
                </li>
                <li>"Anser att prestanda är en viktig del av applikationen"</li>
                <li>"Van vid CI & CD, Container-teknik, Linux och AWS eller dylik IaaS"</li>
                <li>"God kommunikationsförmåga och vilja att samarbeta"</li>
                <li>
                    "Driven att göra applikationen bättre; det gnager om det finns långsiktiga buggar eller tillkortakommanden"
                </li>
                <li>
                    "Att en användare hör av sig om att du löst deras praktiska problem är roligare än att hoppa på nästa JS-ramverk"
                </li>
            </ul>
            <h3>"Meriterande kunskaper"</h3>
            <ul>
                <li>"Djupare kunskap inom UX/UI-design, användartester etc."</li>
                <li>
                    "Grafisk design (om du arbetat mycket med ett grafiskt program så förstår du stora delar av vad vår applikation måste stödja för att designa en skylt)"
                </li>
                <li>
                    "Full-stack eller intresse utöver front-end (vill du arbeta full-stack eller utvecklas inom back-end så finns den möjligheten här)"
                </li>
            </ul>
            <h3>"Basinformation"</h3>
            <dl>
                <dt>"Anställningsform:"</dt>
                <dd>"Heltid"</dd>
                <dt>"Arbetsplats:"</dt>
                <dd>"Tyresö"</dd>
                <dt>"Hybrid:"</dt>
                <dd>"upp till 50 % remote"</dd>
                <dt>"Tillsättning av tjänsten:"</dt>
                <dd>"Omgående"</dd>
                <dt>"Kollektivavtal:"</dt>
                <dd>"Ja"</dd>
            </dl>
            <h3>"Ansök"</h3>
            <p>
                "Skicka ett mail (" <a href="mailto:niclas@instateam.se">"niclas@instateam.se"</a>
                ") eller ring (" <a href="tel:+46705650510">"070-565 05 10"</a>
                ") och nämn vilken roll du söker. Vi sätter då en tid för att talas vid och tar det därifrån."
            </p>
            <p>
                "Kandidater intervjuas löpande så vänligen vänta inte med att söka. Det vore tråkigt om vi hann fylla tjänsten innan vi fick prata med dig också, så skicka din ansökan nu, så kan vi prata om erfarenheter, projekt eller annat du vill dela när vi hörs 👌"
            </p>
            <p class="JobDescription-contact">
                <strong>"Niclas Åhdén"</strong>
                <span>"Tillförordnad IT-chef"</span>
                <a href="mailto:niclas@instateam.se">"niclas@instateam.se"</a>
                <a href="tel:+46705650510">"070-565 05 10"</a>
            </p>
        </section>
    }
}
