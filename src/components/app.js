// This is the main component of our application.
// It consists of navbar and container for the real components.
// TODO:
//  - add router so the visible component depends on URL.
//  - import only used SVG icons from fontawesome
//  - show progress-bar when DB is not ready
//  - catch db errors (show modal)

import "../style";
import "bulma/css/bulma.css";
import "@fortawesome/fontawesome-free/css/solid";
import "@fortawesome/fontawesome-free/css/fontawesome";

import {h, Component} from "preact";
import {loadConfig} from "../config";
import {Navbar, NavbarItem} from "./Navbar";
import Review from "./Review";
import Create from "./Create";
import Db from "../db";


export default class App extends Component {
  constructor(props) {
    super(props);
    this.state = {
      notes: [],
      url: "new"
    };
  }


  componentDidMount() {
    const db = new Db();
    db.open().then(() => {
      this.db = db;
      return loadConfig(db);
    });
    // FIXME: catch

    this.createForm && this.createForm.focus();

    // Bulma requires this to stick navbar to the top and bottom.
    document.body.classList.add("has-navbar-fixed-top");
    document.body.classList.add("has-navbar-fixed-bottom");

    // Ask user for permission to use really persistent storage.
    navigator.storage && navigator.storage.persist();
  }

  onNavigate = url => this.setState({url})


  render() {
    return (
      <div>
        <Navbar url={this.state.url} onChange={this.onNavigate}>
          FHMP
          <NavbarItem url="new" icon="fas fa-bong" text="Add Note" />
          <NavbarItem url="list" icon="fas fa-list" text="List" />
          <NavbarItem url="review" icon="fas fa-seedling" text="Review" />
        </Navbar>
        {this.db &&
          <div class="container">
            {this.state.url === "new" &&
              <Create db={this.db} ref={ref => this.createForm = ref} />
            }
            {this.state.url === "list" &&
              this.state.notes.map(n => <p>{JSON.stringify(n)}</p>)
            }
            {this.state.url === "review" &&
              <Review
                getNote={this.db.getRandomNote}
                updateNote={this.updateNote}
              />
            }
          </div>
        }
      </div>
    );
  }
}
