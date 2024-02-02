#![warn(clippy::pedantic)]

use rlua::{Lua, Result, Value};

fn main() -> Result<()> {
    let lua = Lua::new();

    lua.context(|lctx| {
        lctx.load(
            r"

            -- add library to lua package path to find fennel
            package.path = package.path .. ';./lib/?.lua'
            local fennel = require('fennel').install {}

            -- add plugin directories to both lua and fennel paths
            package.path = package.path .. ';./plugins/?.lua;./plugins/?/init.lua'
            fennel.path = fennel.path .. ';./plugins/?.fnl;./plugins/?/init.fnl'

            -- TODO: automatically load all libs in ./plugins/
            plugins = {
                require('greeter').f,
                require('attack').f,
                require('damage').f,
            }

            print('Plugin count: ' .. #plugins)
            ",
        )
        .exec()?;

        Ok(())
    })?;

    let lines = [
        "why hello there",
        "attack",
        "damage 2d8",
        "damage 3d6",
        "damage d12",
        "goodbye now",
    ];

    for input in lines {
        println!("<--   {input}");

        eval_line(&lua, &input)?
            .iter()
            .for_each(|line| println!("  --> {line}"));
    }

    Ok(())
}

fn eval_line(lua: &Lua, line: &str) -> Result<Vec<String>> {
    lua.context(|lctx| {
        let globals = lctx.globals();

        globals.set("line", line)?;

        lctx.load(
            r"
            responses = {}

            for _, f in ipairs(plugins) do
                local response = f(line)

                if type(response) == 'table' then
                    for _, line in ipairs(response) do
                        table.insert(responses, line)
                    end
                elseif type(response) == 'string' then
                    table.insert(responses, response)
                end
            end
            ",
        )
        .exec()?;

        let responses: Vec<String> = globals.get("responses")?;

        globals.set("line", Value::Nil)?;
        globals.set("responses", Value::Nil)?;

        Ok(responses)
    })
}
