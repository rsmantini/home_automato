use super::super::lcn_config::LcnConfig;
use ecs::components::ActivationState;
use ecs::world::World;
use reqwest::header;
use serde::Serialize;

pub fn process(world: &mut World, config: &LcnConfig, client: &reqwest::blocking::Client) {
    if !has_command_to_execute(world) {
        return;
    }
    let mdl = get_mdl(config, client);
    println!("executor: mdl request result: {:?}", mdl);
    if mdl.is_none() {
        return;
    }
    execute_commands(world, config, mdl.unwrap(), client);
}

fn has_command_to_execute(world: &mut World) -> bool {
    let range = itertools::izip!(
        &world.components.activation_states,
        &world.components.lcn_commands
    )
    .filter_map(|(a, c)| Some((a.as_ref()?, c.as_ref()?)));
    for (state, _) in range {
        if *state == ActivationState::ReadyToRun {
            return true;
        }
    }
    false
}

fn get_mdl(config: &LcnConfig, client: &reqwest::blocking::Client) -> Option<i32> {
    let res = client
        .get(&config.home_url)
        .header(header::CONTENT_TYPE, "application/json")
        .header(header::COOKIE, &config.cookie)
        .send()
        .ok()?
        .text()
        .ok()?;
    parse_mdl(&res)
}

fn execute_commands(
    world: &mut World,
    config: &LcnConfig,
    mdl: i32,
    client: &reqwest::blocking::Client,
) {
    let range = itertools::izip!(
        &mut world.components.activation_states,
        &world.components.lcn_commands
    )
    .filter_map(|(a, c)| Some((a.as_mut()?, c.as_ref()?)));

    for (state, command) in range {
        if *state != ActivationState::ReadyToRun {
            continue;
        }

        let r = LcnCmdRequest {
            mdl,
            id: command.id.to_string(),
            updatedIds: Vec::new(),
        };

        let command_response = client
            .post(&config.command_url)
            .header(header::CONTENT_TYPE, "application/json")
            .header(header::COOKIE, &config.cookie)
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
