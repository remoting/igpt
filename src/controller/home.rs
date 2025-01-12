use crate::handler_registry;
use crate::util::error::Error;
use crate::util::json::Json;

handler_registry!(example_command);
pub fn example_command(_args: Json) -> Result<Json, Error> {
    Err(Error::new(1, "error"))
}
