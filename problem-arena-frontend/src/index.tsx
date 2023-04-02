import React from 'react';
import ReactDOM from 'react-dom/client';
import './index.sass';
import reportWebVitals from './reportWebVitals';
import {createBrowserRouter, RouterProvider} from "react-router-dom";
import NavMenu from "./components/NavMenu";
import NavBar from "./components/NavBar";
import ProblemsList from "./views/ProblemsList";
import ProblemView from "./views/ProblemView";
import ProblemDetailsForm from "./views/ProblemDetailsForm";
import ProblemsListBySuccessRate from "./views/ProblemListBySuccessRate";

const router = createBrowserRouter([{
    path: '/',
    element: <div>Hello world!</div>
}, {
    path: '/problems',
    element: <ProblemsList />
}, {
    path: '/problem/create',
    element: <ProblemDetailsForm />
}, {
    path: '/problem/edit/:id',
    element: <ProblemDetailsForm />
}, {
    path: '/problem/:id',
    element: <ProblemView />
}, {
    path: '/problems-by-success-rate',
    element: <ProblemsListBySuccessRate/>
}]);

const root = ReactDOM.createRoot(
    document.getElementById('root') as HTMLElement
);

root.render(
    <React.StrictMode>
        <NavBar/>
        <div className="columns" style={{height: "100%"}}>
            <div className="column is-2" style={{height: "100%"}}><NavMenu/></div>
            <div className="column box mt-2"><RouterProvider router={router}/></div>
        </div>
    </React.StrictMode>
);

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals();
