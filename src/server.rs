use crate::options::ServerOptions;

type GenericError = Box<dyn std::error::Error>;

pub fn serve(opts: ServerOptions) -> Result<(), GenericError> {
    println!("todo: start a server on {}", opts.addr);
    println!("dictionary: {}", opts.dict);
    Ok(())
}
