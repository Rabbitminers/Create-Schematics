
pub const MESSAGE_OK: &str = "ok";
pub const MESSAGE_SIGNUP_SUCCESS: &str = "Signup successfully";
pub const MESSAGE_LOGIN_SUCCESS: &str = "Login successfully";
pub const MESSAGE_LOGIN_FAILED: &str = "Wrong username or password, please try again";
pub const MESSAGE_USER_NOT_FOUND: &str = "User not found, please signup";
pub const MESSAGE_LOGOUT_SUCCESS: &str = "Logout successfully";
pub const MESSAGE_PROCESS_TOKEN_ERROR: &str = "Error while processing token";
pub const MESSAGE_INVALID_TOKEN: &str = "Invalid token, please login again";
pub const MESSAGE_INTERNAL_SERVER_ERROR: &str = "Internal Server Error";
pub const MESSAGE_EMPTY: &str = "Schematic title or description cannot be empty";
pub const MESSAGE_TOO_LONG: &str = "Schematic title or description is too long";
pub const MESSAGE_UNAUTHORIZED: &str = "You are not authorized to this, try logging in again";
pub const MESSAGE_CAN_NOT_DELETE_DATA: &str = "Can not delete data";
pub const MESSAGE_CAN_NOT_UPDATE_DATA: &str = "Can not update data";
pub const MESSAGE_CAN_NOT_FETCH_DATA: &str = "Can not fetch data";
pub const MESSAGE_UPLOAD_SUCCESS: &str = "Uploaded successfully";
pub const MESSAGE_SCHEMATIC_NOT_FOUND: &str = "Schematic not found";
pub const MESSAGE_COMMENT_NOT_FOUND: &str = "Comment not found";
pub const MESSAGE_INVALID_RATING: &str = "Rating must be between 1 and 5";
pub const MESSAGE_DUPLICATE_COMMENT: &str = "You have already commented this schematic";
pub const MESSAGE_CAN_NOT_DELETE_COMMENT: &str = "You don't have delete to modify this comment";
pub const MESSAGE_CAN_NOT_MODIFY_COMMENT: &str = "You don't have modify to modify this comment";

pub const MESSAGE_TOKEN_MISSING: &str = "Token is missing";

pub const AUTHORIZATION: &str = "Authorization";

pub const EMPTY: &str = "";

pub const IGNORE_ROUTES: [&str; 4] = ["/api/ping", "/api/auth/signup", "/api/auth/login", "/api/schematic/search"];

pub const DEFAULT_PER_PAGE: i64 = 10;

pub const DEFAULT_PAGE_NUM: i64 = 1;

pub const EMPTY_STR: &str = "";
