use crate::{handler_registry, util::{error::Error, json::Json}};

handler_registry!(test1);
pub fn test1(args: Json) -> Result<Json,Error> {
    // let mut data = Json::empty();
    // data.set_j("args", args);
    Ok(args)
   //Err(Error::new(1, "msg"))
}
 