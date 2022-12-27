use std::fs;

use failure::Fallible;
use headless_chrome::{Browser, protocol::page::ScreenshotFormat};

fn main() {
    println!("Hello, world!");
    search_devio();
}

fn search_devio() -> Fallible<()> {
    let browser = Browser::default()?;
    let tab = browser.wait_for_initial_tab()?;
    tab.set_default_timeout(std::time::Duration::from_secs(200));

    tab.navigate_to("https://www.google.com/")?;
    tab.wait_until_navigated()?;
    let jpeg_data = tab.capture_screenshot(ScreenshotFormat::JPEG(Some(75)), None, true)?;
    fs::write("screenshot.jpg", &jpeg_data)?;

    tab.wait_for_element("input[name=q]")?.click()?;
    tab.type_str("DevelopersIO")?;
    let jpeg_data = tab.capture_screenshot(ScreenshotFormat::JPEG(Some(75)), None, true)?;
    fs::write("screenshot1-2.jpg", &jpeg_data)?;

    Ok(())
}