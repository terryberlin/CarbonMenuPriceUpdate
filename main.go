package main

import (
	"io"
	"io/ioutil"
	"log"
	"os"
	"strings"

	"github.com/terryberlin/CarbonMenuPriceUpdate/db"
)

type (
	//Job : Job is a structure for job settings.
	Job struct {
		PLU      int    `db:"plu"`
		PriceOld string `db:"priceOld"`
		PriceNew string `db:"priceNew"`
	}
)

func main() {

	unit_id := 94
	plu := "999"
	priceStandard := "0"
	price := "0"
	oldString := `price: `
	newString := `price: `

	Jobs := []Job{}
	sql := `exec reporting.dbo.getPriceChanges $1`
	errSQL := db.SQLDB().Select(&Jobs, sql, unit_id)
	if errSQL != nil {
		log.Println(errSQL)
	}

	data, err1 := ioutil.ReadFile("test.rs")
	if err1 != nil {
		log.Fatal(err1)
	}

	oldString = `price: ` + priceStandard + `, plu: "` + plu + `",`
	newString = `price: ` + price + `, plu: "` + plu + `",`

	newdata := strings.Replace(string(data), oldString, newString, 1)

	err2 := WriteToFile("test2.rs", newdata)
	if err2 != nil {
		log.Fatal(err2)
	}

}

func WriteToFile(filename string, data string) error {
	file, err := os.Create(filename)
	if err != nil {
		return err
	}
	defer file.Close()

	_, err = io.WriteString(file, data)
	if err != nil {
		return err
	}
	return file.Sync()
}

/*
	data, err := ioutil.ReadFile("myJSON.json")
	if err != nil {
		fmt.Println("File reading error", err)
		return
	}

	oldString := `"plu":"999054","price":0,`
	newString := `"plu":"999054","price":10,`
	newdata := strings.Replace(string(data), oldString, newString, 1)

	oldString = `"plu":"100","price":169`
	newString = `"plu":"100","price":1069`
	newdata = strings.Replace(newdata, oldString, newString, 1)


	// m := map[string]interface{}{}
	// json.Unmarshal(data, &m)

	//data2 := json.Unmarshal(data)

	// x := fmt.Sprint(m)
	// mm := strings.Replace(string(x), oldString, newString, 1)
	// fmt.Println(mm)

	// z, _ := json.Marshal(mm)

	// err1 := WriteToFile("z.json", string(z))
	// if err1 != nil {
	// 	log.Fatal(err1)
	// }

	// for k, v := range m {
	// 	fmt.Printf("key: %v, value: %v\n", k, v)
	// }

	//data, _ = json.Marshal(data)
	//newdata := strings.Replace(string(data), oldString, newString, 1)
	fmt.Println()
	//fmt.Println(newdata)

	err2 := WriteToFile("myJSON.json", newdata)
	if err2 != nil {
		log.Fatal(err2)
	}

*/
