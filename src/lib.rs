use neon::prelude::*;
fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello from md5"))
}
fn md5(mut cx: FunctionContext) -> JsResult<JsString>{
    let path = cx.argument::<JsString>(0)?.value(&mut cx);
    let bytes = std::fs::read(path).unwrap();  // Vec<u8>
    let hash = sha256::digest_bytes(&bytes);
    Ok(cx.string(hash))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;
    cx.export_function("md5", md5)?;
    Ok(())
}
