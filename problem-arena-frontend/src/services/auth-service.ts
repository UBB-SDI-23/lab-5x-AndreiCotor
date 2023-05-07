import {axiosConfigured} from "../config";
import {LoginDTO} from "../model/LoginDTO";
import {AxiosResponse} from "axios";

export const AuthService = {
    login: (username: string, password: string):  Promise<AxiosResponse<LoginDTO>> => {
        return axiosConfigured.post("/login", {username, password});
    },

    register: (username: string, password: string): Promise<AxiosResponse<string>> => {
        return axiosConfigured.post("/register", {username, password});
    },

    confirm: (uuid: string): Promise<AxiosResponse> => {
        return axiosConfigured.get("/register/confirm/" + uuid);
    }
}
