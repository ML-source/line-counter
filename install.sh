#!/bin/sh

target=""
owner="leonovk"
repo="line-counter"
exe_name=""
githubUrl=""
githubApiUrl=""
version=""

get_os(){
    os=$(uname -s | awk '{print tolower($0)}')
    if [ "$os" = "linux" ]; then
        echo "unknown-linux-gnu"
    elif [ "$os" = "darwin" ]; then
        echo "apple-darwin"
    else
        echo $os
    fi
}

get_folder() {
    os=$(uname -s | awk '{print tolower($0)}')
    if [ "$os" = "linux" ]; then
        echo "/usr/bin"
    elif [ "$os" = "darwin" ]; then
        echo "/usr/local/bin"
    else
        echo "/usr/local/bin"
    fi
}

# parse flag
for i in "$@"; do
    case $i in
        -r=*|--repo=*)
            target="${i#*=}"
            shift # past argument=value
        ;;
        -v=*|--version=*)
            version="${i#*=}"
            shift # past argument=value
        ;;
        -e=*|--exe=*)
            exe_name="${i#*=}"
            shift # past argument=value
        ;;
        -g=*|--github=*)
            githubUrl="${i#*=}"
            shift # past argument=value
        ;;
        *)
            # unknown option
        ;;
    esac
done

args=(`echo $target | tr '/' ' '`)

if [ -z "$exe_name" ]; then
    exe_name=$repo
    echo "INFO: file name is not specified, use '$repo'"
    echo "INFO: if you want to specify the name of the executable, set flag --exe=name"
fi

if [ -z "$githubUrl" ]; then
    githubUrl="https://github.com"
fi
if [ -z "$githubApiUrl" ]; then
    githubApiUrl="https://api.github.com"
fi

downloadFolder="${TMPDIR:-/tmp}"
mkdir -p ${downloadFolder} # make sure download folder exists
os=$(get_os)
arch=$(uname -m)
file_name="${exe_name}-${arch}-${os}.tar.gz" # the file name should be download
downloaded_file="${downloadFolder}/${file_name}" # the file path should be download
executable_folder=$(get_folder) # Eventually, the executable file will be placed here

# if version is empty
if [ -z "$version" ]; then
    asset_path=$(
        command curl -L \
            -H "Accept: application/vnd.github+json" \
            -H "X-GitHub-Api-Version: 2022-11-28" \
            ${githubApiUrl}/repos/${owner}/${repo}/releases |
        command grep -o "/${owner}/${repo}/releases/download/.*/${file_name}" |
        command head -n 1
    )
    if [[ ! "$asset_path" ]]; then
        echo "ERROR: unable to find a release asset called ${file_name}"
        exit 1
    fi
    asset_uri="${githubUrl}${asset_path}"
else
    asset_uri="${githubUrl}/${owner}/${repo}/releases/download/${version}/${file_name}"
fi

echo "[1/2] Download ${asset_uri} to ${downloadFolder}"

echo "[2/2] Install ${exe_name} to the ${executable_folder}"
tar -xz -f ${downloaded_file}
mv "line-counter" /usr/bin

echo "${exe_name} was installed successfully to ${exe}"
if command -v $exe_name --version >/dev/null; then
    echo "Run '$exe_name --help' to get started"
fi

exit 0
