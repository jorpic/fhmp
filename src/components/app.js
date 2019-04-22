import "../style";
import "bulma/css/bulma.css";
import "@fortawesome/fontawesome-free/css/solid";
import "@fortawesome/fontawesome-free/css/fontawesome";

import {h, Component} from "preact";
import {Navbar, NavbarItem} from "./Navbar";
import Review from "./Review";
import Create from "./Create";
import Db from "../lib/Db";


export default class App extends Component {
  constructor(props) {
    super(props);
    this.state = {
      listUpdated: false,
      notes: [],
      url: "new"
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

    // Bulma requires this to stick navbar to the top and bottom
    document.body.classList.add("has-navbar-fixed-top");
    document.body.classList.add("has-navbar-fixed-bottom");
  }

  refreshList = () => {
    this.db.getNotes()
      .then(notes => this.setState({listUpdated: false, notes}));
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
          <NavbarItem url="config" icon="fas fa-cog" text="Config" />
        </Navbar>
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
      </div>
    );
  }
}
