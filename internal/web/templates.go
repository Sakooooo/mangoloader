package templates

import (
	"embed"
	"html/template"
	"io"
)

// go:embed templates/
var tmplFS embed.FS

type Template struct {
	templates *template.Template
}

func New() *Template {
	templates := template.Must(template.New("").ParseFS(tmplFS, "templates/*.html"))
	return &Template{
		templates: templates,
	}
}

func (t *Template) Render(w io.Writer, name string, data interface{}) error {
	return t.templates.ExecuteTemplate(w, name, data)
}
