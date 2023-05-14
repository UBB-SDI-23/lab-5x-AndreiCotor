import axios from "axios";

export let axiosConfigured = axios.create({baseURL: "https://problemarena.mooo.com/api"});
//export let axiosConfigured = axios.create({baseURL: "http://localhost:8000/api"});