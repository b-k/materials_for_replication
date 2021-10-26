#![allow(unused_must_use)] //So you know right now this is not production quality.
use std::{env, fs, io::Write, thread, time::Duration};
use thirtyfour_sync::error::WebDriverError;
use thirtyfour_sync::prelude::*;

fn accept_all_cookies() -> Result<i32, WebDriverError> {
    let caps = DesiredCapabilities::firefox();
    let driver = WebDriver::new("http://localhost:4444/wd/hub", &caps)?;
    driver.get("https://link.springer.com/journals/y/1")?
          .query(By::Id("onetrust-accept-btn-handler"))
          .first()?
          .click()?
          .quit()?;
    Ok(0)
}

fn get_jrnls() -> Result<i32, WebDriverError> {
    let caps = DesiredCapabilities::firefox();
    let driver = WebDriver::new("http://localhost:4444/wd/hub", &caps)?;
    accept_all_cookies()?;

    let mut urls = std::fs::File::create("urls")?;
    let mut r: Vec<char> = ('a'..='z').collect();
    r.push('0');

    for i in r.iter() {
      for j in [1, 2] {
        println!("{}", i);
        match driver.get(format!("https://link.springer.com/journals/{}/{}", i, j)) {
            Ok(_) => {
                let jrnls = driver.query(By::ClassName("c-atoz-list__link")).all()?;
                for j in jrnls.iter() {
                    let j_url = j.get_attribute("href")?.expect("N/a");
                    println!("{}", j_url);
                    writeln!(urls, "{}", &j_url);
                }
            }
            Err(_) => continue,
        }
      }
    }
    driver.quit();
    Ok(0)
}

fn get_tab() -> Result<i32, WebDriverError> {
    let caps = DesiredCapabilities::firefox();
    accept_all_cookies()?;

    let mut tab = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("tab")
        .unwrap();
    writeln!(
        tab,
        "name|tags|must_be|will_be|strongly_encourage|encourage"
    )
    .expect("write glitch.");

    //Got our journal list. Now tick through it.

    for j in fs::read_to_string("urls")?.lines() {
        let driver = WebDriver::new("http://localhost:4444/wd/hub", &caps)?;
        eprintln!("{}", j);
        driver.get(j);
        let t = driver.query(By::XPath("//meta[@name='keywords']")).first();
        match t {
            Ok(_) => 0,
            Err(_) => continue,
        };

        let t = t?.get_attribute("content")?.unwrap();
        let tags: Vec<&str> = t.split(", ").collect();
        let name = &tags[0];
        if !driver
            .query(By::LinkText("Submission guidelines"))
            .exists()?
        {
            println!("No submissions from {}.", name);
            continue;
        }
        driver
            .query(By::LinkText("Submission guidelines"))
            .first()?
            .click()?;
        let ps = driver.page_source()?;
        writeln!(
            tab,
            "{}|{:?}|{:?}|{:?}|{:?}|{:?}",
            name,
            t,
            ps.find("data sets must be made freely available"),
            ps.find("all relevant raw data, will be freely available"),
            ps.find("strongly encourages that all datasets"),
            ps.find("encourage authors to ensure that their datasets"),
        )?;
        driver.quit()?;
    }
    Ok(0)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!(
            "I need an argument:
get_jrnls to get the list of journals
get_tab: tabulate each journal"
        );
        return;
    }
    match args[1].as_str() {
        "get_jrnls" => {
            get_jrnls().unwrap();
        }
        "get_tab" => {
            get_tab().unwrap();
        }
        &_ => (),
    }
}
