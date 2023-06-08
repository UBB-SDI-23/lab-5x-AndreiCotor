import {useState} from "react";
import {AdminService} from "../services/admin-service";

export default function AdminPanel() {
    const [info, setInfo] = useState<string>("");
    const [active, setActive] = useState<boolean>(true);
    const [pag, setPag] = useState<number>(0);
    const [error, setError] = useState<string>("");

    const submit = (table: string) => {
        if (window.confirm("Are you sure you want to delete all the data from " + table + "?")) {
            AdminService.deleteAllFromTable(table).then((res) => {
                if (res.status) {
                    setInfo("Table deleted successfully");
                } else {
                    setInfo("An error has occurred");
                }
            }).catch((res) => {
                setInfo("An error has occurred");
            })
        }
    }

    const populate = () => {
        setActive(false);
        setInfo("Running database populate scripts. This may take a while...")
        AdminService.runGenerate();
    }

    const validate = (): boolean => {
        if (pag < 1 || pag > 50) {
            setError("Number of entities must be between 1 and 50!");
            return false;
        }
        else {
            setError("");
            return true;
        }
    }

    const setPagination = () => {
        if (!validate()) {
            return;
        }

        AdminService.setPagOpt(pag).then((res) => {
            setInfo("Successfully set pagination option")
        }).catch((res) => {
            setInfo("There was an error!")
        });
    }

    return (
        <div>
            <h1 className="title">Admin panel</h1>
            <div className="columns">
                <div className="column is-half-desktop">
                    <p>{info}</p>
                    <button className="button is-link mt-2 mr-2" onClick={() => submit("participates")}>
                        Delete all participations
                    </button>
                    <button className="button is-link mt-2 mr-2" onClick={() => submit("submissions")}>
                        Delete all submissions
                    </button>
                    <button className="button is-link mt-2 mr-2" onClick={() => submit("contests")}>
                        Delete all contests
                    </button>
                    <button className="button is-link mt-2 mr-2" onClick={() => submit("problems")}>
                        Delete all problems
                    </button>
                    <button className="button is-link mt-2 mr-2" onClick={() => populate()} disabled={!active}>
                        Run populate script
                    </button>

                    <div className="field">
                        <label className="label">Entities per page</label>
                        <div className="control">
                            <input className="input"
                                   type="number"
                                   placeholder="Rating larger than"
                                   onBlur={() => validate()}
                                   value={pag}
                                   onChange={(e) => setPag(Number(e.target.value))}
                            />
                        </div>
                        <p className="has-text-danger">{error}</p>
                    </div>
                    <button className="button is-link mt-2" onClick={() => setPagination()}>
                        Set pagination
                    </button>
                </div>
            </div>
        </div>
    );
}