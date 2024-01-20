use redis_module::{redis_module, RedisString, RedisResult, Context, RedisError, RedisValue};

fn set_alias(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    ctx.auto_memory();
    if args.len() < 3 {
        return Err(RedisError::WrongArity);
    }

    let (key, aliases, value) = (
        &args[1], 
        &args[2..args.len()-1],
        &args[args.len()-1],
    );

    ctx.open_key_writable(&key).as_string_dma()?.write(value.as_slice())?;
    let key_raw = key.as_slice();
    for alias in aliases {
        ctx.open_key_writable(alias).as_string_dma()?.write(key_raw)?;
    }

    Ok(true.into())
}

fn get_alias(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    ctx.auto_memory();
    if args.len() != 2 {
        return Err(RedisError::WrongArity);
    }

    let alias = ctx.open_key(&args[1]);
    let Some(key_raw) = alias.read()? else {
        return Ok(RedisValue::Null)
    };

    let key = ctx.open_key(&ctx.create_string(key_raw));
    let value = key.read()?.map(|b| std::str::from_utf8(b)).transpose()?;
    Ok(value.into())
}

redis_module! {
    name: "mod_rs",
    version: 1,
    allocator: (redis_module::alloc::RedisAlloc, redis_module::alloc::RedisAlloc),
    data_types: [],
    commands: [
        ["mod_rs.set_alias", set_alias, "write", 0, 0, 0],
        ["mod_rs.get_alias", get_alias, "readonly", 0, 0, 0],
    ],
}
