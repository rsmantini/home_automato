use super::super::components::*;
use super::super::lcn;
use lame_ecs::{component_iter, component_iter_mut, World};
use lcn::LcnClient;
use reqwest::header;
use serde::Serialize;

pub fn process(world: &mut World, client: &LcnClient) {
    if !has_command_to_execute(world) {
        return;
    }
    let mdl = get_mdl(&client.http_client, &client.home_url);
    println!("executor: mdl request result: {:?}", mdl);
    if mdl.is_none() {
        return;
    }
    execute_commands(
        world,
        &client.http_client,
        &client.command_url,
        mdl.unwrap(),
    );
}

fn has_command_to_execute(world: &World) -> bool {
    let range = component_iter!(world, ActivationState, LcnCommand);
    for (state, _, _) in range {
        if *state == ActivationState::ReadyToRun {
            return true;
        }
    }
    false
}

fn get_mdl(client: &reqwest::blocking::Client, home_url: &str) -> Option<i32> {
    let res = client
        .get(home_url)
        .header(header::CONTENT_TYPE, "application/json")
        .send()
        .ok()?
        .text()
        .ok()?;
    parse_mdl(&res)
}

fn execute_commands(
    world: &mut World,
    client: &reqwest::blocking::Client,
    command_url: &str,
    mdl: i32,
) {
    let range = component_iter_mut!(world, ActivationState, LcnCommand);

    for (state, command, _) in range {
        if *state != ActivationState::ReadyToRun {
            continue;
        }

        let r = LcnCmdRequest {
            mdl,
            id: command.id.to_string(),
            updatedIds: Vec::new(),
        };

        let command_response = client
            .post(command_url)
            .header(header::CONTENT_TYPE, "application/json")
            .json(&r)
            .send();
        if command_response.is_ok() {
            *state = ActivationState::ToBeScheduled;
        }
        println!(
            "executor: command request succeeded: {}",
            command_response.is_ok()
        );
        println!(
            "executor: command response check: {:?}",
            check_command_response(command_response)
        );
    }
}

fn parse_mdl(html: &str) -> Option<i32> {
    let offset = 10;
    let i = html.find("mdl")? + offset;
    let mdl: String = html
        .get(i..)?
        .chars()
        .take_while(|c| c.is_digit(10))
        .collect();
    mdl.parse::<i32>().ok()
}

fn check_command_response(
    response: Result<reqwest::blocking::Response, reqwest::Error>,
) -> Option<()> {
    let text = response.ok()?.text().ok()?;
    let token = ":true}}";
    match text.ends_with(token) {
        true => Some(()),
        false => None,
    }
}

#[derive(Serialize, Debug)]
#[allow(non_snake_case)]
struct LcnCmdRequest {
    mdl: i32,
    id: String,
    updatedIds: Vec<String>,
}
