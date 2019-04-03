import "../style";
import "bulma/css/bulma.css";
import "@fortawesome/fontawesome-free/css/solid";
import "@fortawesome/fontawesome-free/css/fontawesome";
import cls from "classnames";

import { h, Component } from "preact";
import { Tab, Tabs } from "./Tabs";
import Db from "../lib/Db";
import Review from "./Review";
import Create from "./Create";



export default class App extends Component {
  constructor(props) {
    super(props);
    this.state = {
      listUpdated: false,
      notes: []
    };

    this.db = new Db();
  }

  componentDidMount() {
    this.createForm && this.createForm.focus();

    // FIXME: hide this
    this.db.idb.Notes.hook("creating", () => {
      this.setState({listUpdated: true});
      // NB. must return `undefined` here, otherwise return value will be used
      // as a primary key for a new object.
    });
  }

  refreshList = () => {
    this.db.getNotes()
      .then(notes => this.setState({listUpdated: false, notes}));
  }


  render() {
    const { listUpdated } = this.state;
    return (
      <div class="section">
        <div class="container">
          <Tabs>
            <Tab icon="fas fa-bong" name="New">
              <Create
                onSave={this.db.createNote}
                ref={ref => this.createForm = ref}/>
            </Tab>
            <Tab
              icon={cls("fas fa-list", {"has-text-danger": listUpdated})}
              name="List"
              onActive={this.refreshList}>
              {this.state.notes.map(n => <p>{JSON.stringify(n)}</p>)}
            </Tab>
            <Tab icon="fas fa-seedling" name="Review">
              <Review
                getNote={this.db.getRandomNote}
                updateNote={this.updateNote} />
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
