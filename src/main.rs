use dotenv::dotenv;
use playwright::{api::Page, Playwright};
use std::{
    io::{stdout, Write},
    thread::sleep,
    time::Duration,
};
use twilio_async::{Twilio, TwilioRequest};

const RETRY_TIMEOUT_SECONDS: u64 = 15;
const FULFILLMENT_BTN_SELECTOR: &str = "div[id^='fulfillment-add-to-cart-button-'] button";

#[tokio::main]
async fn main() -> Result<(), playwright::Error> {
    println!("👷 Initializing...");
    let playwright = Playwright::initialize().await?;
    playwright.prepare()?; // Install browsers
    let chromium = playwright.chromium();
    let browser = chromium.launcher().headless(false).launch().await?;
    let context = browser.context_builder().build().await?;
    let page = context.new_page().await?;
    println!("✅ Successfully initialized!");

    sign_in(&page).await?;
    goto_xbox_url(&page).await?;

    let mut stdout = stdout();
    println!("🏭 Check availability process started!\n");
    loop {
        let is_available = test_fulfillment_button_state(&page).await?;
        if is_available {
            add_to_cart(&page).await?;
            text_my_phone().await.unwrap();
            break;
        }
        print!(
            "\r[{:?}] Xbox Series X is unavailable 😭\n",
            chrono::offset::Utc::now()
        );
        stdout.flush()?;

        for i in 0..=RETRY_TIMEOUT_SECONDS {
            print!("\rRetrying in {}s...", RETRY_TIMEOUT_SECONDS - i);
            stdout.flush()?;
            sleep(Duration::from_secs(1));
        }

        println!("\rRefreshing page...");
        // Reload page before next check
        page.reload_builder().reload().await?;
    }

    Ok(())
}

async fn sign_in(page: &Page) -> Result<(), playwright::Error> {
    dotenv().ok();
    let email = dotenv::var("EMAIL").expect("EMAIL must be set");
    let password = dotenv::var("PASSWORD").expect("PASSWORD must be set");

    println!("📍 Navigating to Best Buy sign in page...");
    page.goto_builder("https://www.bestbuy.com/").goto().await?;

    page.click_builder("button[data-lid='hdr_signin']")
        .click()
        .await?;

    let sign_in_btn_selector = "div[id^='shop-account-menu'] a[data-lid='ubr_mby_signin_b']";
    page.wait_for_selector_builder(sign_in_btn_selector)
        .wait_for_selector()
        .await?;

    page.click_builder(sign_in_btn_selector).click().await?;

    println!("🔒 Signing into Best Buy...");
    page.click_builder("input[type='email']").click().await?;
    page.keyboard.input_text(&email).await?;

    page.click_builder("input[type='password']").click().await?;
    page.keyboard.input_text(&password).await?;

    page.click_builder("button[type='submit']").click().await?;

    // effectively waits for the login process to finish
    page.wait_for_selector_builder("button[data-lid='hdr_signin']")
        .wait_for_selector()
        .await?;

    println!("✅ Successfully signed in to Best Buy!");

    Ok(())
}

async fn goto_xbox_url(page: &Page) -> Result<(), playwright::Error> {
    println!("📍 Navigating to Xbox Series X page...");
    page.goto_builder(
        "https://www.bestbuy.com/site/microsoft-xbox-series-x-1tb-console-black/6428324.p?skuId=6428324"
    )
    .goto()
    .await?;
    println!("✅ Successfully navigated to Xbox Series X page!");

    Ok(())
}

async fn test_fulfillment_button_state(page: &Page) -> Result<bool, playwright::Error> {
    let fulfillment_button = page
        .wait_for_selector_builder(FULFILLMENT_BTN_SELECTOR)
        .wait_for_selector()
        .await?
        .unwrap();

    Ok(fulfillment_button.is_disabled().await.unwrap_or_default() == false)
}

async fn add_to_cart(page: &Page) -> Result<(), playwright::Error> {
    println!("🥳 Xbox Series X is available!");
    page.click_builder(FULFILLMENT_BTN_SELECTOR).click().await?;
    page.wait_for_selector_builder("div[id^='shop-commerce-elements']")
        .wait_for_selector()
        .await?;
    println!("✅ Successfully added Xbox Series X to cart!");

    Ok(())
}

async fn text_my_phone() -> Result<(), String> {
    println!("Texting you about this Great Success...");

    dotenv().ok();
    let twilio_account_id = dotenv::var("TWILIO_ACCOUNT_SID").expect("Failed to parse Account SID");
    let twilio_auth_token = dotenv::var("TWILIO_AUTH_TOKEN").expect("Failed to parse Auth Token");
    let to_number = dotenv::var("TO_NUMBER").expect("Failed to parse to number");
    let twilio_number = dotenv::var("TWILIO_NUMBER").expect("Failed to parse Twilio from number");

    let twilio = Twilio::new(twilio_account_id, twilio_auth_token).unwrap();
    // sending a message
    match twilio
        .send_msg(
            &twilio_number,
            &to_number,
            "Xbox Series X has been added to your cart at https://www.bestbuy.com/cart",
        )
        .run()
        .await
    {
        Ok(_) => {
            println!("✅ Successfully sent SMS!");
            return Ok(());
        }
        Err(_) => return Err(format!("Failed to send text message")),
    }
}
