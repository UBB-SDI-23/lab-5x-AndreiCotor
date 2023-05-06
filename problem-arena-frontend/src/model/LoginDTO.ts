import {axiosConfigured} from "../config";

export interface LoginDTO {
    id: number,
    token: string,
    username: string,
}

export const saveLoginDTO = (loginDTO: LoginDTO) => {
    localStorage.setItem("id", loginDTO.id.toString());
    localStorage.setItem("token", loginDTO.token);
    localStorage.setItem("username", loginDTO.username);
}

export const loadLoginDTO = (): LoginDTO | null => {
    let id = localStorage.getItem("id");
    let token = localStorage.getItem("token");
    let username = localStorage.getItem("username");

    if (!id || !token || !username) {
        return null;
    }

    return {id: Number(id), token, username};
}

export const clearLoginDTO = () => {
    localStorage.removeItem("id");
    localStorage.removeItem("token");
    localStorage.removeItem("username");
}