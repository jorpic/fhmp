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

import cls from "classnames";
import {h, Component} from "preact";
import {loadConfig} from "../config";
import {Navbar, NavbarItem} from "./Navbar";
import Review from "./Review";
import Create from "./Create";
import Config from "./Config";
import Db from "../db";


export default class App extends Component {
  constructor(props) {
    super(props);
    this.state = {
      notes: [],
      url: "new",
      db: null,
      message: null,
    };
  }


  componentDidMount() {
    const db = new Db();
    db.open()
      .then(() => loadConfig(db))
      // Notify that db is ready only after config is loaded.
      // To prevent using config before it is ready.
      .then(() => this.setState({db}))
      .catch(() => this.onMessage({error: true, msg: "Loading form DB failed"}));

    // Bulma requires this to stick navbar to the top and bottom.
    document.body.classList.add("has-navbar-fixed-top");
    document.body.classList.add("has-navbar-fixed-bottom");

    // Ask user for permission to use really persistent storage.
    navigator.storage && navigator.storage.persist();
  }


  onNavigate = url => this.setState({url})

  // Show important messages on a modal form.
  onMessage = message => this.setState({message})
  clearMessage = () => this.setState({message: null})


  render() {
    const {db, url, message} = this.state;
    return (
      <div>
        <Navbar url={url} onChange={this.onNavigate}>
          FHMP
          <NavbarItem url="new" icon="fas fa-bong" text="Add Note" />
          <NavbarItem url="list" icon="fas fa-list" text="List" />
          <NavbarItem url="review" icon="fas fa-seedling" text="Review" />
          <NavbarItem url="config" icon="fas fa-cog" text="Config" />
        </Navbar>
        {db &&
          <div class="container">
            {url === "new" &&
              <Create db={db} onMessage={this.onMessage} />
            }
            {this.state.url === "review" &&
              <Review db={db} onMessage={this.onMessage} />
            }
            {this.state.url === "config" &&
              <Config db={db} onMessage={this.onMessage} />
            }
          </div>
        }
        {message &&
          <div class="modal is-active">
            <div class="modal-background" onClick={this.clearMessage} />
            <div class="modal-content" onClick={this.clearMessage}>
              <article
                class={cls("message", {
                  "is-danger": message.error,
                  "is-warning": message.warning,
                  "is-success": message.success})}
              >
                <div class="message-body">{message.msg}</div>
              </article>
            </div>
          </div>
        }
      </div>
    );
  }
}
