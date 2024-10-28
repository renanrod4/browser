use web_view::*;

fn main() {
    let html_content = r#"
        <!doctype html>
        <html>
            <head>
                <style>
                    body, html { margin: 0; padding: 0; }
                    body {
                        display: flex;
                        flex-direction: column; /* Organiza a barra e o iframe em coluna */
                        height: 100vh; /* Ocupa toda a altura da janela */
                    }
                    #nav-bar {
                        position: absolute;
                        top: 10px; left: 50%;
                        background: #333;
                        padding: 4px;
                        padding-right:0;
                        display: flex;
                        align-items: center;
                        border-radius: 9px;
                        text-align: center;
                        width: 30%;
                        transform: translateX(-50%);
                        box-shadow: 0.2rem 0.8rem 1.6rem var(--colorPrimary600);

                    }
                    #nav-bar.hidden {
                        display: none; /* Oculta a barra quando a classe 'hidden' é adicionada */
                    }
                    #nav-bar input {
                        width: 85%;
                        height:100%;
                        padding: 8px;
                        outline: none;
                        border:none;
                        border-radius:6px;
                        font-size:25px;

                    }
                    #content {
                        width: 100%;
                        flex: 1; /* Permite que o iframe ocupe o espaço restante */
                        border: none;
                    }
                    #nav-btn{
                    height:100%;
                    width:15%;
                    color:white;
                    
                    background:#333;
                    border:none;
                    padding:8px;
                    font-size:25px;

                    }
                </style>
            </head>
            <body>
                <div id="nav-bar">
                    <input id="url" value="http://localhost:80"  type="text" placeholder="Enter URL" oninput="preserveInitialText()"  />
                    <button id="nav-btn" onclick="navigate()">Go</button>
                </div>
                <iframe id="content" src="https://localhost:80"></iframe>
                
                <script>
                    function navigate() {
                        var url = document.getElementById('url').value;
                        document.getElementById('content').src = "http://localhost:"+url.replace("http://localhost:","");
                        document.getElementById('nav-bar').classList.add('hidden'); 
                    }

                    
                    document.getElementById('url').addEventListener('keypress', function(event) {
                        if (event.key === 'Enter') {
                            navigate();
                        }
                    });

                    
                    document.addEventListener('keydown', function(event) {
                        if (event.ctrlKey && event.key === 'l') {
                            event.preventDefault(); 
                            var navBar = document.getElementById('nav-bar');
                            navBar.classList.remove('hidden'); 
                            document.getElementById('url').focus(); 
                        }
                    });
                    
                    
                    document.getElementById('content').addEventListener('click', function() {
                        var navBar = document.getElementById('nav-bar');
                        if (navBar.classList.contains('hidden')) {
                            navBar.classList.remove('hidden'); 
                            document.getElementById('url').focus(); 
                        }
                    });

                    
                    document.addEventListener('keydown', function(event) {
                        if (event.key === 'Tab') {
                            var navBar = document.getElementById('nav-bar');
                            if (navBar.classList.contains('hidden')) {
                                navBar.classList.remove('hidden'); 
                                document.getElementById('url').focus(); 
                            }
                        }
                    });
                    function preserveInitialText() {
                        initialText = "http://localhost:"
                        const input = document.getElementById("url");
                        if (!input.value.startsWith(initialText)) {
                            input.value = initialText + input.value.slice(initialText.length);
                        }
                    }

                </script>
            </body>
        </html>
    "#;

    web_view::builder()
        .title("Custom Web Viewer")
        .content(Content::Html(html_content))
        .size(1878, 1040)
        .resizable(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();
}
