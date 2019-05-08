// Create component provides textarea to enter a note and some logic to save
// it to storage.
// Drafts are saved to IDB after a small timeout.
// TODO:
//  - preview rendered markdown
//  - add tags and plugins
//  - use conditional rendereing for save button

import cls from "classnames";
import {h, Component} from "preact";
import config from "../config";


export default class Create extends Component {
  constructor(props) {
    super(props);
    this.state = {
      text: "",
      draftSaved: true
    };
  }

  componentDidMount() {
    // load draft from IDB
    this.props.db.getDraft()
      .then(draft =>
        draft !== undefined && this.setState({text: draft.text}))
      .catch(() => this.props.onMessage({
        warning: true,
        msg: "Failed to load draft from storage"
      }));

    // save draft every now and then
    this.draftSaveLoop = setInterval(
      () => this.state.draftSaved
        || this.props.db.saveDraft(this.state.text)
          .then(() => this.setState({draftSaved: true})),
      config.DRAFT_SAVE_TIMEOUT);
  }

  componentWillUnmount() {
    clearInterval(this.draftSaveLoop);
    this.props.db.saveDraft(this.state.text)
      .catch(() => this.props.onMessage({
        warning: true,
        msg: "Failed to save draft"
      }));
  }


  onText = ev => this.setState({
    text: ev.target.value,
    draftSaved: false,
  })


  onSave = () => {
    const {text} = this.state;
    this.props.db.createNote(text)
      .then(() => {
        this.setState({text: ""});
        this.props.onMessage({
          success: true,
          msg: "Message saved sucessfully."
        });
      })
      .catch(err => this.props.onMessage({
        error: true,
        msg: (
          <span>
            Sorry! <br />
            We could not save your note to the local storage. <br />
            {err}
          </span>)
      }));
  }


  render() {
    const textareaCls = cls(
      "textarea has-extra-height",
      {"is-success": this.state.draftSaved});

    return (
      <div class="section">
        <div class="field">
          <textarea class={textareaCls}
            ref={ref => this.textarea = ref}
            onInput={this.onText}
            value={this.state.text}
            autofocus={true}
          />
        </div>

        <nav class="navbar is-light is-fixed-bottom">
          <div class="navbar-brand">
            <a class="navbar-item is-expanded has-text-centered"
              disabled={!this.state.text}
              onClick={this.onSave}
            >
              Save
            </a>
          </div>
        </nav>
      </div>);
  }
}
