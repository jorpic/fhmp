
import cls from "classnames";
import {h, Component} from "preact";
import config from "../config";
import Page from "./Page";


export default class Sync extends Component {
  constructor(props) {
    super(props);
    this.state = {
      log: []
    }
  }

  componentDidMount() {
    const log = msg => this.setState({log: this.state.log.concat(msg)});
    if (!config.SYNC_SERVER_URL) {
      log("Sync server is not defined in config.");
    } else {
      log("Sync started");
      const {db} = this.props;
      db.pushToServer()
        .then(() => {
          log("  - data sent");
          return db.pullFromServer()
        })
        .then(stats => log("  - data received"))
        .catch(err => log("Sync failed\n   - " + err));
    }
  }

  render() {
    return (
      <Page>
        <pre>{this.state.log.join("\n")}</pre>
      </Page>
    );
  }
}
