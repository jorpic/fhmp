import { h, Component } from 'preact';
import Card from 'preact-material-components/Card';
import 'preact-material-components/Card/style.css';
import 'preact-material-components/Button/style.css';
import style from './style';
import { auth, googleAuthProvider } from '../../firebase';


export default class SignIn extends Component {
  // returns promise with `{user}` as a result.
  static check() {
    return new Promise(function(resolve, reject) {
      auth.onAuthStateChanged(resolve);
    });
  }


  signIn() {
    auth.signInWithRedirect(googleAuthProvider)
  }


  render() {
    return (
      <div class={`${style.signIn} page`}>
        <h1>Sign In</h1>
        <Card>
          <div class={style.cardBody}>
            You need to sign in to proceed.
          </div>
          <Card.Actions>
            <Card.ActionButton onClick={this.signIn}>
              Sign In
            </Card.ActionButton>
          </Card.Actions>
        </Card>
      </div>
    );
  }
}
