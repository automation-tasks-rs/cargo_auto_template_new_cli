# github_release_prepare.sh

printf "\033[0;33m    Prepare template.tar.gz to upload manually to the latest GitHub release. \033[0m\n"

printf "\033[0;33m    Use git archive to tar the folder cargo_auto_template_new_cli. \033[0m\n"
mkdir -p tmp
git archive -o tmp/template.tar HEAD
printf "\033[0;33m    Delete directories and files not meant for upload. \033[0m\n"
tar -f tmp/template.tar --delete docs 2> /dev/null
tar -f tmp/template.tar --delete target 2> /dev/null
tar -f tmp/template.tar --delete Cargo.lock 2> /dev/null
tar -f tmp/template.tar --delete github_release_prepare.sh 2> /dev/null
gzip -f tmp/template.tar

printf "\n"
printf "\033[0;33m    Prepare automation_tasks_rs.tar.gz to upload manually to the latest GitHub release. \033[0m\n"

printf "\033[0;33m    Replace string automation--tasks--rs in files. \033[0m\n"
sed -i "s/automation--tasks--rs/automation-tasks-rs/g" "automation_tasks_rs/README.md";
sed -i "s/automation--tasks--rs/automation-tasks-rs/g" "automation_tasks_rs/src/main.rs";
sed -i "s/github_bestia_dev_api_oauth2_ssh_1/github_api_oauth2_ssh_1/g" "automation_tasks_rs/github_api_config.json";

printf "\033[0;33m    Compress to tar.gz \033[0m\n"
tar -C automation_tasks_rs --exclude='target' --exclude='.file_hashes.json' --exclude='Cargo.lock' -cf tmp/automation_tasks_rs.tar .
gzip -f tmp/automation_tasks_rs.tar

printf "\033[0;33m    Replace back strings automation--tasks--rs in files. \033[0m\n"
sed -i "s/automation-tasks-rs/automation--tasks--rs/g" "automation_tasks_rs/README.md";
sed -i "s/automation-tasks-rs/automation--tasks--rs/g" "automation_tasks_rs/src/main.rs";
sed -i "s/github_api_oauth2_ssh_1/github_bestia_dev_api_oauth2_ssh_1/g" "automation_tasks_rs/github_api_config.json";

printf "\n"
printf "\033[0;33m    Now you can upload the files \033[0m\n"
printf "\033[0;33m    tmp/automation_tasks_rs.tar.gz and template.tar.gz \033[0m\n"
printf "\033[0;33m    to the latest GitHub release. \033[0m\n"
printf "\033[0;32m https://github.com/automation-tasks-rs/cargo_auto_template_new_cli/releases \033[0m\n"
printf "\n"
printf "\033[0;33m    After the upload you can delete the files: \033[0m\n"
printf "\033[0;32m rm tmp/template.tar.gz \033[0m\n"
printf "\033[0;32m rm tmp/automation_tasks_rs.tar.gz \033[0m\n"
printf "\n"

