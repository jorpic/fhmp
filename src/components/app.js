// This is the main component of our application.
// It consists of navbar and container for the real components.
// TODO:
//  - add router so the visible component depends on URL.
//  - import only used SVG icons from fontawesome
//  - show progress-bar when DB is not ready

import "../style";
import "bulma/css/bulma.css";
import "@fortawesome/fontawesome-free/css/solid";
import "@fortawesome/fontawesome-free/css/fontawesome";

import cls from "classnames";
import {h, Component} from "preact";
import {Router, route} from "preact-router";
import {loadConfig} from "../config";

import Review from "./Review";
import ListNotes from "./ListNotes";
import EditNote from "./EditNote";
import Config from "./Config";
import Db from "../db";


export default class App extends Component {
  constructor(props) {
    super(props);
    this.state = {
      db: null,
    };
  }

  componentDidMount() {
    const db = new Db();
    db.open()
      .then(() => loadConfig(db))
      // Notify that db is ready only after config is loaded.
      // To prevent using config before it is ready.
      .then(() => this.setState({db}))
      .catch(err => this.onMessage({error: true, err, msg: "Loading form DB failed"}));

    // Bulma requires this to stick navbar to the top and bottom.
    document.body.classList.add("has-navbar-fixed-top");
    document.body.classList.add("has-navbar-fixed-bottom");

    // Ask user for permission to use really persistent storage.
    navigator.storage && navigator.storage.persist();
  }


  render() {
    const {db, url, message} = this.state;
    if (!db) {
      // FIXME: empty page just to be able to handle error notifications
      // during DB loading.
      return "Loading...";
    }
    return (
      <Router>
        <Default path="/" to="/list" />
        <ListNotes path="/list" db={db} />
        <EditNote path="/new" db={db} />
        <EditNote path="/edit/:noteId" db={db} />
        <Review path="/review" db={db} />
        <Config path="/config" db={db} />
      </Router>
    );
  }
}


class Default extends Component {
  componentWillMount() {
    route(this.props.to, true);
  }

  render() { return null; }
}
