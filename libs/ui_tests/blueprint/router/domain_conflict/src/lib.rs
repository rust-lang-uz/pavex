use pavex::blueprint::{router::GET, Blueprint};
use pavex::f;

pub fn handler() -> pavex::response::Response {
    todo!()
}

pub fn blueprint() -> Blueprint {
    let mut bp = Blueprint::new();
    bp.domain("{*any}.company.com").nest({
        let mut bp = Blueprint::new();
        bp.route(GET, "/", f!(crate::handler));
        bp
    });
    bp.domain("{sub}.company.com").nest({
        let mut bp = Blueprint::new();
        bp.route(GET, "/", f!(crate::handler));
        bp
    });
    bp
}
