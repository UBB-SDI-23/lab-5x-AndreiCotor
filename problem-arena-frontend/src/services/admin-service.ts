import {AxiosResponse} from "axios/index";
import {axiosConfigured} from "../config";

export const AdminService = {
    changeRole: (id: number, role: string): Promise<AxiosResponse> => {
        return axiosConfigured.put("/update-role", {id, role}, {headers: { Authorization: `Bearer ${localStorage.getItem("token")}`}});
    },

    deleteAllFromTable: (table: string): Promise<AxiosResponse> => {
        return axiosConfigured.delete("/all-" + table, {headers: { Authorization: `Bearer ${localStorage.getItem("token")}`}});
    },

    runGenerate: () => {
        axiosConfigured.get("/run-generate", {headers: { Authorization: `Bearer ${localStorage.getItem("token")}`}});
    }
}