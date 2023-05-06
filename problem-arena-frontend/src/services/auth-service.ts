import {axiosConfigured} from "../config";
import {LoginDTO} from "../model/LoginDTO";
import {AxiosResponse} from "axios";

export const AuthService = {
    login: (username: string, password: string):  Promise<AxiosResponse<LoginDTO>> => {
        return axiosConfigured.post("/login", {username, password});
    }
}
