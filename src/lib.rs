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

    ctx.open_key_writable(&key).write(value.try_as_str()?)?;
    let key_str = key.try_as_str()?;
    for alias in aliases {
        ctx.open_key_writable(alias).write(key_str)?;
    }

    Ok(true.into())
}

fn get_alias(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    ctx.auto_memory();
    if args.len() != 2 {
        return Err(RedisError::WrongArity);
    }

    let alias = ctx.open_key(&args[1]);
    let Some(key) = alias.read()? else {
        return Ok(RedisValue::Null)
    };

    let value = ctx.open_key(&ctx.create_string(key))
        .read()?
        .map(|b| b.to_vec());
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
