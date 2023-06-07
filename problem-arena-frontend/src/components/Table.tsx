import {useNavigate} from "react-router-dom";
import RatingDisplay from "./RatingDisplay";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faPenToSquare, faTrash} from "@fortawesome/free-solid-svg-icons";
import {useContext} from "react";
import {AuthContext} from "../contexts/AuthContext";

interface TableProps {
    columns: string[],
    properties: string[],
    elements: any[],
    path: string,
    creator?: string,
    uid?: string,
    user?: string,
    deleteFunction: (id: string) => void
}

export default function Table(props: TableProps) {
    const navigate = useNavigate();
    const { authContext } = useContext(AuthContext);

    function extractObjectProperties(properties: string[], obj: any) {
        let tsx_list = properties.map((prop, index) => {
                if (prop !== "rating") {
                    return (<td data-label={props.columns[index]} onClick={() => navigate(props.path + "/" + obj.id)}>
                        { obj[prop] }
                    </td>);
                }
                else {
                    return (<td className="is-progress-cell" data-label="Rating" onClick={() => navigate(props.path + "/" + obj.id)}>
                        <RatingDisplay rating={obj[prop]}/>
                    </td>);
                }
        });

        if (props.creator && props.uid) {
            tsx_list.push(
                <td data-label="Creator">
                    <a href={"/user/" + obj[props.uid]}>{obj[props.creator]}</a>
                </td>
            );
        }

        return tsx_list;
    }



    const tableRows = props.elements.map((el, index) => {
        return (<tr key={index}>
            <td onClick={() => navigate(props.path + "/" + el.id)}>{typeof el.id === "string"? index + 1: el.id}</td>
            { extractObjectProperties(props.properties, el) }
            { (authContext && (authContext.role !== "regular" || (props.uid && authContext.id === el[props.uid])))?
                (<td className="is-actions-cell">
                    <div className="buttons">
                        <button className="button is-danger" onClick={() => props.deleteFunction(String(el.id))}>
                            <FontAwesomeIcon icon={faTrash} />
                        </button>
                        <button className="button is-link edit" onClick={() => navigate(props.path + "/edit/" + el.id)}>
                            <FontAwesomeIcon icon={faPenToSquare} />
                        </button>
                    </div>
                </td>): (<td></td>)}
        </tr>);
    });

    const tableHeader = props.columns.map((el) => {
        return (
            <th>{ el }</th>
        );
    });

    if ((props.creator && props.uid) || props.user) {
        tableHeader.push(<th>Username</th>)
    }

    return (
        <div className="b-table">
            <div className="table-wrapper has-mobile-cards">
                <table className="table is-fullwidth is-striped is-hoverable is-fullwidth">
                    <thead>
                    <tr>
                        <th><abbr title="Index">#</abbr></th>
                        {tableHeader}
                        <th>Actions</th>
                    </tr>
                    </thead>
                    <tbody>
                        {(tableRows.length > 0)? tableRows: (<p>No data to show.</p>)}
                    </tbody>
                </table>
            </div>
        </div>
    );
}