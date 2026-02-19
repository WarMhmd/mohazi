flag = true;

if (data.id === undefined || data.id === null) {

    
    result.errors.push({
        path: "login.id",
        message: ""Invalid ID.""
    });
    flag = false;
    

} else if (
    typeof data.id !== "object" ||
    typeof data.id.name !== "string" ||
    typeof data.id.size !== "number"
) {

    result.errors.push({
        path: "login.id",
        message: ""Invalid ID.""
    });
    flag = false;

} else {

    

    

    

    

    

    

    

    

    if (flag) {
        successData.id = data.id;
    }
}
