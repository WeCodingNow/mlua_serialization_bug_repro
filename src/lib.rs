use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct SimpleStruct {
    pub foo: i64,
}

#[derive(Serialize, Deserialize, Debug)]
struct SimpleStructContainer {
    pub simple_args: std::vec::Vec<SimpleStruct>,
}

use mlua::prelude::*;

#[mlua::lua_module]
fn repro(lua: &Lua) -> LuaResult<LuaTable> {
    let module = lua.create_table()?;

    module.set(
        "call_function_with_container",
        lua.create_function(|lua, fn_name: String| -> mlua::Result<()> {
            let container_with_vector = SimpleStructContainer {
                simple_args: vec![SimpleStruct { foo: 1 }],
            };

            // checking metatable on simple_args inside container_with_vector
            // which should be nil, because it should serialize to just a table

            let serialized_container_with_vector = lua.to_value(&container_with_vector)?;

            if let mlua::Value::Table(tbl) = serialized_container_with_vector {
                println!("outer tbl metatable: {:?}", tbl.get_metatable());

                let inner_tbl: mlua::Value = tbl
                    .get("simple_args")?;

                println!("outer_tbl.simple_args: {:?}", inner_tbl);

                if let mlua::Value::Table(inner_tbl_content) = inner_tbl {
                    println!("simple_args content: {:?}", inner_tbl_content);

                    for pair in inner_tbl_content
                        .clone()
                        .pairs::<mlua::Value, mlua::Value>()
                    {
                        println!("simple_args pair: {:?}", pair);
                    }

                    let inner_tbl_metatable = inner_tbl_content.get_metatable().unwrap();
                    println!("inner_tbl metatable: {:?}", inner_tbl_metatable);

                    for pair in inner_tbl_metatable.pairs::<mlua::Value, mlua::Value>() {
                        let pair = pair.unwrap();
                        println!("metatable pair: {:?}", pair);

                        if let mlua::Value::String(k) = pair.0 {
                            println!("k: {:?}", k.to_str().unwrap());
                        };

                        if let mlua::Value::Boolean(v) = pair.1 {
                            println!("v: {:?}", v);
                        };
                    }
                }
            };

            lua.globals()
                .get::<_, mlua::Function>(fn_name)?
                .call::<_, ()>(lua.to_value(&container_with_vector))?;

            Ok(())
        })?,
    )?;

    Ok(module)
}
