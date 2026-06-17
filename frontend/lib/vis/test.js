
import { validateUuid, getUuidVersion } from "./utils/uuid_validator.js";



export const validateBirth = async (data) => {
    let result = {
        success: false,
        errors: [],
        data: data
    };

    const successData = {};
    let flag = true;
    {
        flag = true; flag = true;

        if (data.childName === undefined || data.childName === null) {
            result.errors.push({
                path: "childName",
                message: "\"Child name is invalid\""
            });
            flag = false;
        } else if (typeof data.childName !== "string") {
            result.errors.push({
                path: "childName",
                message: "\"Child name is invalid\""
            });
            flag = false;
        } else {
            if (data.password.length < 8) {
                result.errors.push({
                    path: "password",
                    message: "Password must be at least 8 characters long."
                });
                flag = false;
            }
            if (data.password.length > 100) {
                result.errors.push({
                    path: "password",
                    message: "Password must be at most 100 characters long."
                });
                flag = false;
            }
            if (!new RegExp("^(?=.*[A-Za-z])(?=.*\\d)[A-Za-z\\d]+$").test(data.password)) {
                result.errors.push({
                    path: "password",
                    message: "Password must contain at least one letter and one number."
                });
                flag = false;
            }
            if (flag) {
                successData.password = data.password;
            }
        }

    }
    {
        flag = true;
        if (data.username === undefined || data.username === null) {
            result.errors.push({
                path: "username",
                message: "Invalid value."
            });
            flag = false;
        } else if (typeof data.username !== "string") {
            result.errors.push({
                path: "username",
                message: "Invalid value."
            });
            flag = false;
        } else {
            if (data.username.length < 3) {
                result.errors.push({
                    path: "username",
                    message: "Username must be at least 3 characters long."
                });
                flag = false;
            }
            if (data.username.length > 20) {
                result.errors.push({
                    path: "username",
                    message: "Username must be at most 20 characters long."
                });
                flag = false;
            }
            if (!new RegExp("^[a-zA-Z0-9_]+$").test(data.username)) {
                result.errors.push({
                    path: "username",
                    message: "Username can only contain letters, numbers, and underscores."
                });
                flag = false;
            }
            if (flag) {
                successData.username = data.username;
            }
        }

    }
=======
    if (data.childName.length < 3) {
        result.errors.push({
            path: "childName",
            message: "Child name cannot be shorter than 3 characters."
        });
        flag = false;
    }
    if (data.childName.length > 20) {
        result.errors.push({
            path: "childName",
            message: "Child name cannot be longer than 20 characters."
        });
        flag = false;
    }
}if (flag) {
            successData.childName = data.childName;
        }
    }
    {
        flag = true;flag = true;

if (data.dateOfBirth === undefined || data.dateOfBirth === null) {
    result.errors.push({
        path: "dateOfBirth",
        message: "\"Invalid date of birth\""
    });
    flag = false;
} else if (typeof data.dateOfBirth !== "string") {
    result.errors.push({
        path: "dateOfBirth",
        message: "\"Invalid date of birth\""
    });
    flag = false;
} else {

    if (flag) {
        const dateObj = new Date(data.dateOfBirth);
        if (isNaN(dateObj.getTime())) {
            result.errors.push({
                path: "dateOfBirth",
                message: "\"Invalid date of birth\""
            });
            flag = false;
        } else {

            if (flag) {
                let formatted = "YYYY-mm-dd";
                const yyyy = dateObj.getFullYear().toString();
                const yy = yyyy.slice(-2);
                const M = dateObj.getMonth() + 1;
                const MM = M.toString().padStart(2, '0');
                const monthNamesShort = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
                const monthNamesFull = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
                const MMM = monthNamesShort[dateObj.getMonth()];
                const MMMM = monthNamesFull[dateObj.getMonth()];

                const d = dateObj.getDate();
                const dd = d.toString().padStart(2, '0');

                const start = new Date(dateObj.getFullYear(), 0, 0);
                const diff = dateObj - start;
                const oneDay = 1000 * 60 * 60 * 24;
                const DDD = Math.floor(diff / oneDay).toString().padStart(3, '0');

                const H = dateObj.getHours();
                const HH = H.toString().padStart(2, '0');

                let h = H % 12;
                h = h ? h : 12;
                const hh = h.toString().padStart(2, '0');

                const m = dateObj.getMinutes();
                const mm = m.toString().padStart(2, '0');

                const s = dateObj.getSeconds();
                const ss = s.toString().padStart(2, '0');

                const a = H >= 12 ? 'PM' : 'AM';

                formatted = formatted.replace(/YYYY|yyyy|YY|yy|MMMM|MMM|MM|M|DDD|dd|d|HH|H|hh|h|mm|m|ss|s|a/g, match => {
                    switch(match) {
                        case 'YYYY': case 'yyyy': return yyyy;
                        case 'YY': case 'yy': return yy;
                        case 'MMMM': return MMMM;
                        case 'MMM': return MMM;
                        case 'MM': return MM;
                        case 'M': return M.toString();
                        case 'DDD': return DDD;
                        case 'dd': return dd;
                        case 'd': return d.toString();
                        case 'HH': return HH;
                        case 'H': return H.toString();
                        case 'hh': return hh;
                        case 'h': return h.toString();
                        case 'mm': return mm;
                        case 'm': return m.toString();
                        case 'ss': return ss;
                        case 's': return s.toString();
                        case 'a': return a;
                        default: return match;
                    }
                });
                data.dateOfBirth = formatted;
            }
        }
    }
}
if (flag) {
            successData.dateOfBirth = data.dateOfBirth;
        }
    }
    {
        flag = true;flag = true;
>>>>>>> Alhareth-types

    if (data.id === undefined || data.id === null) {
        result.errors.push({
            path: "id",
            message: "\"Invalid UUID\""
        });
        flag = false;
    } else if (typeof data.id !== "string") {
        result.errors.push({
            path: "id",
            message: "\"Invalid UUID\""
        });
        flag = false;
    } else {
<<<<<<< HEAD
        if (!new RegExp("^[\\w-\\.]+@([\\w-]+\\.)+[\\w-]{2,4}$").test(data.email)) {
            result.errors.push({
                path: "email",
                message: "Please enter a valid email address."
            });
            flag = false;
        }
        if (flag) {
            successData.email = data.email;
        }
    }

}
{
    flag = true;
    if (data.password === undefined || data.password === null) {
        result.errors.push({
            path: "password",
            message: "Invalid value."
        });
        flag = false;
    } else if (typeof data.password !== "string") {
        result.errors.push({
            path: "password",
            message: "Invalid value."
        });
        flag = false;
    } else {
        if (data.password.length < 8) {
            result.errors.push({
                path: "password",
                message: "Password must be at least 8 characters long."
            });
            flag = false;
        }
        if (data.password.length > 100) {
            result.errors.push({
                path: "password",
                message: "Password must be at most 100 characters long."
            });
            flag = false;
        }
        if (!new RegExp("^(?=.*[A-Za-z])(?=.*\\d)[A-Za-z\\d]+$").test(data.password)) {
            result.errors.push({
                path: "password",
                message: "Password must contain at least one letter and one number."
            });
            flag = false;
        }
        if (flag) {
            successData.password = data.password;
        }
    }

}
{
    flag = true;
    if (data.username === undefined || data.username === null) {
        result.errors.push({
            path: "username",
            message: "Invalid value."
        });
        flag = false;
    } else if (typeof data.username !== "string") {
        result.errors.push({
            path: "username",
            message: "Invalid value."
        });
        flag = false;
    } else {
        if (data.username.length < 3) {
            result.errors.push({
                path: "username",
                message: "Username must be at least 3 characters long."
            });
            flag = false;
        }
        if (data.username.length > 20) {
            result.errors.push({
                path: "username",
                message: "Username must be at most 20 characters long."
            });
            flag = false;
        }
        if (!new RegExp("^[a-zA-Z0-9_]+$").test(data.username)) {
            result.errors.push({
                path: "username",
                message: "Username can only contain letters, numbers, and underscores."
            });
            flag = false;
        }
        if (flag) {
            successData.username = data.username;
        }
    }

}
if (data.id.length !== 36) {
    result.errors.push({
        path: "id",
        message: "\"Invalid UUID\""
    });
    flag = false;
}
if (!/^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12}$/.test(data.id)) {
    result.errors.push({
        path: "id",
        message: "\"Invalid UUID\""
    });
    flag = false;
}
}
if (flag) {
    successData.id = data.id;
}
    }

if (result.errors.length === 0) {
    result = {
        success: true,
        errors: [],
        data: successData
    };
}

return result;
};
