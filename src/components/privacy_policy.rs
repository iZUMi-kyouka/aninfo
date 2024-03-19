use yew::prelude::*;

use crate::{components::appcontext_provider::AppContext, utils::handle_theme};

#[function_component(PrivacyPolicy)]
pub fn privacy_policy() -> Html {
    let app_ctx = use_context::<AppContext>().unwrap();

    html! {
        <div class={format!("privacy-policy {}", handle_theme(&app_ctx))}>
        <h1>{"Privacy Policy for ANiNFO"}</h1>

        <p>{"ANiNFO respects your privacy and is committed to protecting any information you provide while using our application. This Privacy Policy outlines how ANiNFO handles information collected from users."}</p>

        <h2>{"Information Collection and Use"}</h2>

        <p>{"ANiNFO does not collect any personal user data. We do not require you to provide any personal information to use our application."}</p>

        <p>{"However, ANiNFO utilizes Google Fonts API to enhance the visual experience of our application. When you use ANiNFO, your device may automatically send certain information to Google, including, but not limited to, your device's IP address and the date and time of your request. This information is solely used by Google to deliver the requested font resources and is subject to Google's Privacy Policy."}</p>

        <h2>{"Children's Privacy"}</h2>

        <p>{"ANiNFO does not knowingly collect any personally identifiable information from children under the age of 13. If you are a parent or guardian and you are aware that your child has provided us with personal information, please contact us so that we can take appropriate action."}</p>

        <h2>{"Changes to This Privacy Policy"}</h2>

        <p>{"ANiNFO may update this Privacy Policy from time to time. Thus, you are advised to review this page periodically for any changes. We will notify you of any changes by posting the new Privacy Policy on this page. These changes are effective immediately after they are posted on this page."}</p>

        <h2>{"Contact Us"}</h2>

        <p>{"If you have any questions or suggestions about our Privacy Policy, do not hesitate to contact us at "}<a href={"mailto:contact@aninfo.com"}>{"contact@aninfo.com"}</a>{"."}</p>

        <p>{"This Privacy Policy was last updated on March 18, 2024."}</p>
        </div>}
    }
