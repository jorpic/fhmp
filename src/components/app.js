import { h, Component } from 'preact';
import { Router } from 'preact-router';

import Spinner from './Spinner';
import SignIn from './SignIn';
import Header from './header';
import Home from '../routes/home';
import Profile from '../routes/profile';
import NotFound from '../routes/404';

export default class App extends Component {
  constructor(props) {
    super(props);
    this.state = {
      signIn: 'checking'
    };
    SignIn.check()
      .then(result => {
        const signed = result && result.user;
        this.setState({
          user: signed ? result.user : null,
          signIn: signed ? "signed" : "not signed"
        });
      })
      .catch(() => this.setState({signIn: 'failed'}));
  }


  handleRoute = e => {
    this.setState({
      currentUrl: e.url
    });
  };


  render() {
    return (
      <div id="app">
        <Header
            selectedRoute={this.state.currentUrl}
            message={this.state.signIn}
        />
        {this.state.signIn === 'checking'
          ? <Spinner />
          : this.state.signIn !== 'signed'
          ? <SignIn />
          : (
            <Router onChange={this.handleRoute}>
              <Home path="/" />
              <Profile path="/profile/" user="me" />
              <Profile path="/profile/:user" />
              <NotFound default />
            </Router>
          )
        }
      </div>
    );
  }
}
