pipeline {
    agent any

    triggers {
        pollSCM ('*/5 * * * *')
    }

    stages {
        stage ('build') {
            steps {
                sh './scripts/build.sh'
            }
        }
        
        stage ('test') {
            steps {
                sh 'cargo test'
            }
        }
    }
}