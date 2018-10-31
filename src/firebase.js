import firebase from 'firebase';

  // Initialize Firebase
const config = {
    apiKey: "AIzaSyBx6w-MlCdGoMnT_ZPIqGa0ORRx3O72Vsk",
    authDomain: "fhmp-firebase.firebaseapp.com",
    databaseURL: "https://fhmp-firebase.firebaseio.com",
    projectId: "fhmp-firebase",
    storageBucket: "fhmp-firebase.appspot.com",
    messagingSenderId: "567830048648"
};

firebase.initializeApp(config);

export const database = firebase.database();
export const auth = firebase.auth();
export const googleAuthProvider = new firebase.auth.GoogleAuthProvider();
