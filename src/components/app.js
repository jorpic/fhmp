import "../style";
import "bulma/css/bulma.css";
import "@fortawesome/fontawesome-free/css/solid";
import "@fortawesome/fontawesome-free/css/fontawesome";
import cls from "classnames";

import { h, Component } from "preact";
import Dexie from "dexie";
import {Tab, Tabs} from "./Tabs";


export default class App extends Component {
  constructor(props) {
    super(props);
    this.state = {
      text: "",
      listUpdated: false,
      notes: []
    };
  }

  componentDidMount() {
    this.textarea && this.textarea.focus();
    const db = new Dexie("fhmp");
    db.version(1).stores({
      Notes: "++id,createTime" //, text"
    });
    db.Notes.hook("creating", () => {
      this.setState({listUpdated: true});
      // NB. must return undefined here, otherwise return value will be used
      // as a primary key for new object.
    });
    db.open();
    this.db = db;
  }

  onText = ev => this.setState({text: ev.target.value})
  saveText = () => {
    const createTime = new Date().toISOString();
    this.db.Notes.add({createTime, text: this.state.text})
      .then(() => this.setState({text: ""}));
  }

  refreshList = () => {
    this.db.Notes.toArray()
      .then(notes => this.setState({listUpdated: false, notes}));
  }

  render() {
    return (
      <div class="section">
        <div class="container">
          <Tabs>
            <Tab icon="fas fa-bong" name="New">
              <div class="field">
                <textarea class="textarea"
                  ref={ref => this.textarea = ref}
                  onInput={this.onText}
                  value={this.state.text}/>
              </div>
              <div class="buttons">
                <button class="button is-primary"
                  disabled={this.state.text == ""}
                  onClick={this.saveText}>
                  Save
                </button>
              </div>
            </Tab>
            <Tab icon={cls("fas fa-list", {"has-text-danger": this.state.listUpdated})}
              name="List"
              onActive={this.refreshList}>
              {this.state.notes.map(n => <p>{JSON.stringify(n)}</p>)}
            </Tab>
            <Tab icon="fas fa-cog">
              Config
            </Tab>
          </Tabs>
        </div>
      </div>
    );
  }
}
