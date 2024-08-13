pipeline {
    agent any
    environment {
        BRANCH_NAME = "${env.GIT_BRANCH.split('/').size() > 1 ? env.GIT_BRANCH.split('/')[1..-1].join('/') : env.GIT_BRANCH}"
    }

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
                echo BRANCH_NAME
                sh './scripts/deploy.sh $BRANCH_NAME'
            }
        }
    }

    post {
        always {
            cleanWs()
        }
    }
}