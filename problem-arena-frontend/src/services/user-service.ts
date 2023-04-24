import {axiosConfigured} from "../config";
import {AxiosResponse} from "axios";
import {PaginationDTO, StatisticPagination} from "../model/PaginationDTO";
import {NewUser, User, UserReportDTO} from "../model/user";

export const UserService = {
    getUsers: (pagination: PaginationDTO): Promise<AxiosResponse<User[]>> => {
        return axiosConfigured.get("/user", {params: pagination});
    },

    getUser: (id: string): Promise<AxiosResponse<User>> => {
        return axiosConfigured.get("/user/" + id);
    },

    updateUser: (user: User): Promise<AxiosResponse> => {
        return axiosConfigured.put("/user", user);
    },

    addUser: (user: NewUser): Promise<AxiosResponse> => {
        return axiosConfigured.post("/user", user);
    },

    deleteUser: (id: string): Promise<AxiosResponse> => {
        return axiosConfigured.delete("/user/" + id);
    },

    getUsersAutocomplete: (last_name: string): Promise<AxiosResponse<User[]>> => {
        return axiosConfigured.get("/user/autocomplete", {params: {lname: last_name}})
    },

    getProblemsByParticipations: (pagination: StatisticPagination): Promise<AxiosResponse<UserReportDTO[]>> => {
        return axiosConfigured.get("/user-by-number-of-participations", {params: pagination});
    },
}