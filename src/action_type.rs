use insert::Insert;
use auth::Auth;
use select::Select;

pub enum ActionType {
    Auth(Auth),
    Select(Select),
    Insert(Insert),
}
