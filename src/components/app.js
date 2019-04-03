import "../style";
import "bulma/css/bulma.css";
import "@fortawesome/fontawesome-free/css/solid";
import "@fortawesome/fontawesome-free/css/fontawesome";
import cls from "classnames";

import { h, Component } from "preact";
import Dexie from "dexie";
import { Tab, Tabs } from "./Tabs";
import Review from "./Review";
import Create from "./Create";


export default class App extends Component {
  constructor(props) {
    super(props);
    this.state = {
      listUpdated: false,
      notes: []
    };
  }

  componentDidMount() {
    this.createForm && this.createForm.focus();

    const db = new Dexie("fhmp");
    db.version(1).stores({
      Notes: "++id,createTime" //, text"
    });
    db.Notes.hook("creating", () => {
      this.setState({listUpdated: true});
      // NB. must return `undefined` here, otherwise return value will be used
      // as a primary key for a new object.
    });
    db.open();
    this.db = db;
  }

  refreshList = () => {
    this.db.Notes.toArray()
      .then(notes => this.setState({listUpdated: false, notes}));
  }

  // FIXME: take oldest 100 by last acess time, select random among them
  getRandomNote = () => this.db.Notes.toArray()
    .then(ns => {
      const i = Math.floor(ns.length * Math.random());
      return Promise.resolve(ns[i]);
    })


  // FIXME: extract storage API
  // - initDB() -> db
  // - getRandomNote(db) -> note
  // - getAllNotes(db, tags) -> [note]
  // - addReview(db, id, review) -> result
  // - createNote(db) -> id
  // - editNote(db, id, note) -> result
  //
  // handle "unable to save" errors
  //

  createNote = text => {
    const createTime = new Date().toISOString();
    return this.db.Notes.add({createTime, text});
  }

  render() {
    const { listUpdated } = this.state;
    return (
      <div class="section">
        <div class="container">
          <Tabs>
            <Tab icon="fas fa-bong" name="New">
              <Create
                onSave={this.createNote}
                ref={ref => this.createForm = ref}/>
            </Tab>
            <Tab
              icon={cls("fas fa-list", {"has-text-danger": listUpdated})}
              name="List"
              onActive={this.refreshList}>
              {this.state.notes.map(n => <p>{JSON.stringify(n)}</p>)}
            </Tab>
            <Tab icon="fas fa-seedling" name="Review">
              <Review getNote={this.getRandomNote} updNote={this.updateNote}/>
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
