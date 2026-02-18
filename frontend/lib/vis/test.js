flag = true;

if (data.username === undefined || data.username === null) {

    
    result.errors.push({
        path: "login.username",
        message: ""Invalid username.""
    });
    flag = false;
    

} else if (typeof data.username !== "string") {

    result.errors.push({
        path: "login.username",
        message: ""Invalid username.""
    });
    flag = false;

} else {
    if (data.username.length < 3) {
        result.errors.push({
            path: "login.username",
            message: "Username must be at least 3 characters."
        });
        flag = false;
    }
    if (data.username.length > 4) {
        result.errors.push({
            path: "login.username",
            message: "This is maxLength error"
        });
        flag = false;
    }
    if (!/^[a-zA-Z0-9_]+$/.test(data.username)) {
        result.errors.push({
            path: "login.username",
            message: "Username can only contain letters, numbers, and underscores."
        });
        flag = false;
    }
    data.username = data.username.trim();
    data.username = data.username.toUpperCase();
    data.username = data.username.split(",");
    data.username = Number(data.username);
    if (Number.isNaN(data.username)) {
        result.errors.push({
            path: "login.username",
            message: ""Invalid username.""
        });
        flag = false;
    }

    if (flag) {
        successData.username = data.username;
    }
}
