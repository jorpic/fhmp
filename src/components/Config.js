import cls from "classnames";
import {h, Component} from "preact";
import {saveConfig, default as config} from "../config";


export default class Config extends Component {
  constructor(props) {
    super(props);
    this.state = this.defaultState();
  }


  // set state from global `config` variable
  defaultState = () => Object.assign({}, config)


  onCancel = () => this.setState(this.defaultState())


  onSave = () => saveConfig(this.props.db, this.state)
    .then(() => this.props.onMessage({
      success: true,
      msg: "Config was sucessfully saved to storage."}))
    .catch(() => this.props.onMessage({
      error: true,
      msg: "Failed to save config to storage."}))


  isValidNumber = str => {
    if (String(str).match(/^\d+$/) === null)
      return "Should be positive number";
    return null;
  };


  setTimeout = ev => this.setState({DRAFT_SAVE_TIMEOUT: ev.target.value})
  setURL     = ev => this.setState({SYNC_SERVER_URL: ev.target.value})
  setKey     = ev => this.setState({CLIENT_KEY: ev.target.value})
  setQLimit  = ev => this.setState({QUEUE_LIMIT: ev.target.value})


  render() {
    return (
      <div class="section">
        <Field
          autofocus="true"
          name="Draft save timeout (in µseconds)"
          value={this.state.DRAFT_SAVE_TIMEOUT}
          valid={this.isValidNumber}
          onInput={this.setTimeout}
        />

        <Field
          name="Review queue size limit"
          value={this.state.QUEUE_LIMIT}
          onInput={this.setQLimit}
        />

        <Field
          name="Sync server URL"
          value={this.state.SYNC_SERVER_URL}
          onInput={this.setURL}
        />

        <Field
          name="Client key"
          value={this.state.CLIENT_KEY}
          onInput={this.setKey}
        />

        <nav class="navbar is-light is-fixed-bottom">
          <div class="navbar-brand">
            <a class="navbar-item is-expanded has-text-centered"
              onClick={this.onCancel}
            >
              Cancel
            </a>
            <a class="navbar-item is-expanded has-text-centered"
              onClick={this.onSave}
            >
              Save
            </a>
          </div>
        </nav>
      </div>);
  }
}


function Field({name, value, valid, onInput, autofocus}) {
  const err = valid && valid(value);
  return (
    <div class="field">
      <label class="label">{name}</label>
      <div class="control">
        <input
          class={cls("input", {"is-danger": err})}
          value={value}
          onInput={onInput}
          autofocus={autofocus}
        />
      </div>
      <p class={cls("help", {"is-danger": err})}>{err}</p>
    </div>);
}
