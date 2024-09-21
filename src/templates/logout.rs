/// Get the HTML content to render the logout page.
///
/// # See Also
///
/// - This page is served as a response for the `/logout` entry point.
///
/// # Returns
///
/// A `String` version of the HTML, CSS and JS content.
pub fn get_content() -> String {
    r###"<!DOCTYPE html>
<html lang="en">
<head>
    <meta http-equiv="Content-Type" content="text/html; charset=utf-8">
    <title>SysMonk - System Monitor - v{{ version }}</title>
    <meta property="og:type" content="SystemMonitor">
    <meta name="keywords" content="Rust, Monitor, actix, JavaScript, HTML, CSS">
    <meta name="author" content="Vignesh Rao">
    <meta content="width=device-width, initial-scale=1" name="viewport">
    <!-- Favicon.ico and Apple Touch Icon -->
    <link rel="icon" href="https://thevickypedia.github.io/open-source/images/logo/actix.ico">
    <link rel="apple-touch-icon" href="https://thevickypedia.github.io/open-source/images/logo/actix.png">
    <style>
        /* Google fonts with a backup alternative */
        @import url('https://fonts.googleapis.com/css2?family=Ubuntu:wght@400;500;700&display=swap');
        * {
            font-family: 'Ubuntu', 'PT Serif', sans-serif;
        }
        img {
            display: block;
            margin-left: auto;
            margin-right: auto;
        }

        :is(h1, h2, h3, h4, h5, h6) {
            text-align: center;
            color: #F0F0F0;
        }

        button {
            width: 100px;
            margin: 0 auto;
            display: block;
        }

        body {
            background-color: #151515;
        }
    </style>
    <noscript>
        <style>
            body {
                width: 100%;
                height: 100%;
                overflow: hidden;
            }
        </style>
        <div style="position: fixed; text-align:center; height: 100%; width: 100%; background-color: #151515;">
            <h2 style="margin-top:5%">This page requires JavaScript
                to be enabled.
                <br><br>
                Please refer <a href="https://www.enable-javascript.com/">enable-javascript</a> for how to.
            </h2>
            <form>
                <button type="submit" onClick="<meta httpEquiv='refresh' content='0'>">RETRY</button>
            </form>
        </div>
    </noscript>
</head>
<body>
<h2 style="margin-top:5%">LOGOUT</h2>
<h3>{{ detail }}</h3>
<p>
    <img src="https://thevickypedia.github.io/open-source/images/gif/blended_fusion.gif"
        onerror="this.src='https://vigneshrao.com/open-source/images/gif/blended_fusion.gif'"
        width="200" height="200" alt="Image" class="center">
</p>
{% if show_login %}
    <button style="text-align:center" onClick="window.location.href = '/';">LOGIN</button>
{% else %}
    <h3>Please close the session window</h3>
{% endif %}
<h4>Click <a href="https://vigneshrao.com/contact">ME</a> to reach out.</h4>
</body>
<!-- control the behavior of the browser's navigation without triggering a full page reload -->
<script>
    document.addEventListener('DOMContentLoaded', function() {
        history.pushState(null, document.title, location.href);
        window.addEventListener('popstate', function (event) {
            history.pushState(null, document.title, location.href);
        });
    });
</script>
</html>
"###.to_string()
}
