
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
        flag = true;flag = true;

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