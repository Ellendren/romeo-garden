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
                sh './scripts/test.sh'
            }
        }

        stage ('deploy') {
            steps {
                sh './scripts/deploy.sh'
            }
        }
    }

    post {
        always {
            cleanWS()
        }
    }
}