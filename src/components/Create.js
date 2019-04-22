// Create component provides textarea to enter a note and some logic to save
// it to storage.
// Drafts are saved to IDB after a small timeout.
// TODO:
//  - preview rendered markdown
//  - add tags and plugins
//  - use conditional rendereing for save button

import cls from "classnames";
import {h, Component} from "preact";


export default class Create extends Component {
  constructor(props) {
    super(props);
    this.state = {
      error: null,
      text: "",
      draftSaved: true
    };
  }

  componentDidMount() {
    // load draft from IDB
    this.props.db.getDraft()
      .then(draft => draft !== undefined
        && this.setState({text: draft.text}));

    // save draft every now and then
    // FIXME: handle "can't save" errors?
    const DRAFT_SAVE_TIMEOUT = 4; // seconds
    this.draftSaveLoop = setInterval(
      () => this.state.draftSaved
        || this.props.db.saveDraft(this.state.text)
          .then(() => this.setState({draftSaved: true})),
      DRAFT_SAVE_TIMEOUT*1000);
  }

  componentWillUnmount() {
    clearInterval(this.draftSaveLoop);
    this.props.db.saveDraft(this.state.text);
  }


  // called by a parent to focus the textarea
  focus() {
    this.textarea.focus();
  }


  onText = ev =>
    this.setState({
      text: ev.target.value,
      draftSaved: false,
    })


  onSave = () => {
    const {text} = this.state;
    this.props.db.createNote(text)
      .then(() => this.setState({text: ""}))
      .catch(error => this.setState({error}));
  }

  dismissError = () => this.setState({error: null})


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
          />
        </div>
        {this.state.error &&
          <div class="notification is-warning" onClick={this.dismissError}>
            <button class="delete" onClick={this.dismissError} />
            Sorry!
            <br />
            We could not save your note to the local storage: {this.state.error}.
          </div>
        }
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
      </div>
    );
  }
}
